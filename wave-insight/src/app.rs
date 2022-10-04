// use std::time::Instant;

use std::process;
use hashbrown::HashMap;
use actix_web::{App, HttpResponse, HttpServer, get, HttpRequest, web};
use actix::{Actor, StreamHandler, Addr};
// use actix::{Actor, StreamHandler, ActorContext};
use actix_web_actors::ws::{WebsocketContext, self, Message};
use actix_web_static_files::ResourceFiles;
use colored::Colorize;
use uuid::Uuid;





struct WsRequest {
    cmd: String,
    data: String,
}
impl WsRequest {
    fn new(cmd: &str, data: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            data: data.to_string(),
        }
    }
    fn from_str(text: &str) -> Self {
        let text = text.trim();
        let mut splitn_result = text.splitn(2, ".");
        let cmd = splitn_result.next().unwrap();
        let data = splitn_result.next();
        if let Some(data) = data {
            Self::new(cmd, data)
        } else {
            Self::new(cmd, "")
        }
    }
}

lazy_static::lazy_static!{
    static ref CMD_MAP: HashMap<&'static str, fn(addr: Addr<WsConn>, data: String)> = HashMap::new();
}

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct SayHello {
//     pub from: &'static str,
//     pub data: String,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Say {
//     pub from: &'static str,
//     pub data: String,
// }



struct WsConn {
    id: Uuid
}

impl Actor for WsConn {
    type Context = WebsocketContext<Self>;

    /// 连接上
    fn started(&mut self, _: &mut Self::Context) {
        println!("{} join!", self.id);
        // println!("{:?}", ctx.address());
    }

    /// 断开连接
    fn stopped(&mut self, _: &mut Self::Context) {
        println!("{} exit!", self.id);
    }
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(Message::Text(text)) => {
                ctx.text(text)
            },
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

#[get("/ws")]
async fn websocket(
    // params: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
) -> HttpResponse {
    
    // let mut map = req.app_data::<HashMap<&str, &str>>().unwrap();
    let conn = WsConn {
        id: Uuid::new_v4()
    };
    // let mut my_data = data.lock().unwrap();
    let resp = actix_web_actors::ws::start(conn, &req, stream);
    match resp {
        Ok(ret) => ret,
        Err(e) => e.error_response(),
    }
}

/// 启动服务器
pub(crate) async fn create_app() {
    let (addr, port) = ("0.0.0.0", 8080);
    // let map: HashMap<&str, &str> = HashMap::new();

    match HttpServer::new(move || {
        let generated = generate::generate();
        App::new()
            .service(websocket)
            .service(ResourceFiles::new("/", generated))
        }).bind(format!("{}:{}", addr, port))
        {
            Ok(server)=>{
                if port==80{
                    println!("{} Listen at http://{}", "Running:".green().bold(), addr);
                }else{
                    println!("{} Listen at http://{}:{}", "Running:".green().bold(), addr, port);
                }
                let _ = server.run().await;
            },
            Err(e) =>{
                eprintln!("{} {}", "error:".red().bold(), e);
                process::exit(1);
            }
        };
        
}