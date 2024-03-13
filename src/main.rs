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
use serde_json::Value;

use crate::server::ServerMessageX;

mod server;
mod session;


async fn index() -> impl Responder {
    NamedFile::open_async("./gptui/dist/gptui/browser/index.html").await.unwrap()
}

async fn ask_question(req:HttpRequest, _json:web::Json<Value>, srv: web::Data<Addr<server::ChatServer>>) -> impl Responder {
    let client_id = req.match_info().get("client_id").unwrap();
    println!("The requested client_id is:{}", client_id);
    println!("The request body:{}", _json);

    let msg = ServerMessageX{ client_id: client_id.to_owned(), msg: _json.to_string()  };

    // Message handle based message type
    let _ = srv.get_ref().clone().try_send(msg);

    web::Json(())
}

/// Entry point for our websocket route
async fn chat_route(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::ChatServer>>) -> Result<HttpResponse, Error> {
    let client_type = req.match_info().get("client_type").unwrap();

    let _srv = srv.get_ref().clone();

    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: client_type.to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .service(web::resource("/").to(index))  //load html 
            .service(web::resource("/send/{client_id}").to(ask_question))   //send message to client_id
            .route("/count", web::get().to(get_count))
            .route("/ws/{client_type}", web::get().to(chat_route))
            .service(Files::new("/", "./gptui/dist/gptui/browser/"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("192.168.22.66", 4133))?
    .run()
    .await
}


//https://github.com/actix/actix-web/discussions/2729s