use actix::prelude::*;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Duration;

struct MyWebSocket {}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {}
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl Handler<ServerEvent> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ServerEvent, ctx: &mut Self::Context) {
        ctx.text(msg.event);
    }
}

#[derive(Message)]
struct RegisterWSClient {
    addr: Addr<MyWebSocket>,
}

#[derive(Message)]
struct ServerEvent {
    event: String,
}

struct ServerMonitor {
    listeners: Vec<Addr<MyWebSocket>>,
}

impl Actor for ServerMonitor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(5), |act, _| {
            for l in &act.listeners {
                l.do_send(ServerEvent{ event: String::from("Event:") });
            }
        });
    }
}

impl Handler<RegisterWSClient> for ServerMonitor {
    type Result = ();

    fn handle(&mut self, msg: RegisterWSClient, _: &mut Context<Self>) {
        self.listeners.push(msg.addr);
    }
}

fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<ServerMonitor>>,
) -> Result<HttpResponse, Error> {
    let (addr, res) = ws::start_with_addr(MyWebSocket {}, &r, stream)?;

    data.get_ref().do_send(RegisterWSClient { addr: addr });

    Ok(res)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let sys = actix_rt::System::new("example");

    let srvmon = ServerMonitor { listeners: vec![] }.start();

    HttpServer::new(move || {
        App::new()
            .data(srvmon.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
    })
    .bind("127.0.0.1:8080")?
    .start();

    sys.run()
}