use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use actix::*;
use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use serde_json::{json, Value};
use actix_cors::Cors;

use crate::server::ServerMessageX;

mod server;
mod session;
mod db;


async fn index() -> impl Responder {
    NamedFile::open_async("./gptui/dist/gptui/browser/index.html").await.unwrap()
}

/// Entry point for our websocket route
async fn chat_route(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::ChatServer>>) -> Result<HttpResponse, Error> {
    let client_id = req.match_info().get("client_id").unwrap();

    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: client_id.to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Displays state
async fn get_chats(req: HttpRequest, _db: web::Data<sled::Db>) -> impl Responder {
    //let _user_id = req.match_info().get("client_id").unwrap();

    // query database for user


    web::Json( 
        [   
            json!({"dev": "hello, siri"}),
            json!({"dev": "to write a websocket client in rust"}),
            json!({"dev": "how old are you?"}),
            json!({"dev": "what is your name"}),
            json!({"dev": "how old are you?"}),
            json!({"dev": "what is your name"}),
            json!({"dev": "where do you go tomorrow?"}),
            json!({"dev": "help me to design a logo with sunlight"}),
            json!({"dev": "How many people in China?"}),
            json!({"dev": "Why do we learn the rust language?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "help me to design a logo with sunlight"}),
            json!({"dev": "How many people in China?"}),
            json!({"dev": "Why do we learn the rust language?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "help me to design a logo with sunlight"}),
            json!({"dev": "How many people in China?"}),
            json!({"dev": "Why do we learn the rust language?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "help me to design a logo with sunlight"}),
            json!({"dev": "How many people in China?"}),
            json!({"dev": "Why do we learn the rust language?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "How many people in China?"}),
            json!({"dev": "Why do we learn the rust language?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "How far from Shanghai to Beijing?"}),
            json!({"dev": "help me to design a logo with sunlight"}),
        ]
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    // db for chat
    let _db_: sled::Db = sled::open("chat_db").unwrap();

    log::info!("starting HTTP server at http://127.0.0.1:5678");

    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        //.send_wildcard()
        //.allowed_origin("*") // Allow requests from specific origin
        .allowed_methods(vec!["GET", "POST"])  // Allow specific HTTP methods
        .max_age(3600); 

        App::new()
            .wrap(cors)
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(_db_.clone()))
            .service(web::resource("/").to(index))  //load html 
            .route("/chats", web::post().to(get_chats))
            .route("/ws/{client_id}/", web::get().to(chat_route))
            .service(Files::new("/", "./gptui/dist/gptui/browser/"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 4133))?
    .run()
    .await
}


//https://github.com/actix/actix-web/discussions/2729s