//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.

use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use serde_json::Value;
use sled::Db;
use crate::db;
use uuid::Uuid;
/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub room: String,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to specific room
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    pub msgid: String,
    /// Peer message
    pub msg: String,
    /// Room name
    pub userid: String,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client ID
    pub id: usize,

    /// Room name
    pub name: String,
}

/// todo http api with socket
/// 
/// Handler for Message message.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessageX {
    pub client_id: String,
    /// Peer message
    pub msg: String,
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat session.
///
/// Implementation is very naïve.
#[derive(Debug, Clone)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    pub rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
    db: sled::Db
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>, _db:&sled::Db) -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            rooms:HashMap::new(),
            rng: rand::thread_rng(),
            visitor_count,
            db: _db.clone(),
        }
    }
}

impl ChatServer {
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        println!("send msg from room:{}, msg:{}", room, message);
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {

        // notify all users in same room
        //self.send_message("user", "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to main room
        //self.rooms.entry("user".to_owned()).or_default().insert(id);
        let mut room_id = HashSet::new();
        room_id.insert(id);
        self.rooms.insert(msg.room, room_id);

        //let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        //self.send_message("user", &format!("Total visitors {count}"), 0);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {

        // Save user chat request
        println!("current client msg is, msgid:{},msg:{},room:{}", msg.msgid, msg.msg, msg.userid);
 
        // Handle client chat request
        if msg.userid != "chatgpt" {
            let _json_msg_str = format!(r#"{{
                                        "msgid":"{}",
                                        "msg":{}
                                    }}
                                    "#, &msg.msgid, &msg.msg);
            
            // save user chat request into meilisearch document
            let user_db = self.db.open_tree(&msg.userid).unwrap();

            let _json_msg:Value = serde_json::from_str(&_json_msg_str).unwrap();
            user_db.insert(&msg.msgid.as_bytes(), msg.msg.as_bytes()).unwrap();
            let _ = user_db.flush();

            let chatgpt_db = self.db.open_tree("chatgpt").unwrap();
            chatgpt_db.insert(&msg.msgid.as_bytes(), &*msg.userid.as_bytes()).unwrap();
            let _ = chatgpt_db.flush();

            // send message to chatgpt
            self.send_message("chatgpt", &_json_msg_str, 0);
        }

        // Handle chatgpt response
        if msg.userid == "chatgpt" {
            let chatgpt_db = self.db.open_tree("chatgpt").unwrap();
            //find this message related to user
            let msg_json:Value = serde_json::from_str(&msg.msg).unwrap();
            let userid = chatgpt_db.get(&msg_json["msgid"].to_string().trim_matches('"').as_bytes()).unwrap().unwrap();
            let _user = String::from_utf8_lossy(userid.as_ref());
            println!("use key {} to find user {}", &msg_json["msgid"], _user);
            self.send_message(&_user, &msg.msg.as_str(), 0);
        }
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        self.rooms.entry(name.clone()).or_default().insert(id);

        self.send_message(&name, "Someone connected", id);
    }
}


impl Handler<ServerMessageX> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ServerMessageX, _: &mut Context<Self>) {
        
    }
}