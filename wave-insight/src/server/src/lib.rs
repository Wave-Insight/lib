use actix_web::{HttpRequest, HttpResponse, get, web};

use crate::ws_conn::WsConn;
#[macro_use]
mod codegen;
mod ws_conn;
mod cmd;
mod router;
mod handler;



#[get("/ws/{nick}")]
async fn index(
    params:  web::Path<String>,
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