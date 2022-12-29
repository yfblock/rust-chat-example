mod messages;
mod conn;

use actix_web::{web::{self, Path}, Error, HttpRequest, HttpResponse, get};
use actix_web_actors::ws;

use crate::api::ws::conn::WsConnection;

#[get("/ws/{user_id}")]
pub async fn index(req: HttpRequest, stream: web::Payload, user_id: Path<u64>) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsConnection::new(*user_id), &req, stream);
    resp
}