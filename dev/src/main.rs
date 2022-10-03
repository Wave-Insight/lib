use std::time::Instant;

use actix_web::{App, HttpResponse, HttpServer, get, HttpRequest, web};
use actix::{Actor, StreamHandler, ActorContext};
use actix_web_actors::ws::{WebsocketContext, self, Message};

pub struct WsConn {
    pub nick: String,
}



impl Actor for WsConn {
    type Context = WebsocketContext<Self>;

    /// 连接上
    fn started(&mut self, _: &mut Self::Context) {
        println!("{} join!", self.nick);
    }

    /// 断开连接
    fn stopped(&mut self, _: &mut Self::Context) {
        println!("{} exit!", self.nick);
    }
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(Message::Text(text)) => ctx.text(text),
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

#[get("/ws/{nick}")]
async fn index(
    params: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
) -> HttpResponse {
    let conn = WsConn {
        nick: params.to_string(),
    };
    let resp = actix_web_actors::ws::start(conn, &req, stream);
    match resp {
        Ok(ret) => ret,
        Err(e) => e.error_response(),
    }
}

/// 启动服务器
pub async fn create_app() {
    let (addr, port) = ("0.0.0.0", "8080");

    let _ = HttpServer::new(move || {
        App::new()
            .service(index)
    })
        .bind(format!("{}:{}", addr, port))
        .expect(&format!("Can't bind to port {}", port))
        .run()
        .await;
}

#[actix_web::main]
async fn main() {
    create_app().await;
}