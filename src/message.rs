use actix::Message;


#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage {
    pub client_id: String,
    pub msg: String,
}

