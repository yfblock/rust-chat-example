use std::fs;

use actix_easy_multipart::{MultipartForm, text::Text, tempfile::Tempfile};
use actix_web::{Responder, get, post, web};
use rbatis::rbdc::uuid::Uuid;
use serde::Deserialize;

use crate::{db::{record::Record, user::User}, response::{Response, DefaultRes}};

#[derive(Deserialize)]
pub struct GetRecord {
    pub from_user_id: u64,
    pub to_user_id: u64
}

#[get("/get_user_record")]
pub async fn get_user_record(data: web::Query<GetRecord>) -> impl Responder {
    type Json = Response<Vec<Record>>;
    let records = Record::get_records(data.from_user_id, data.to_user_id).await;
    Json::ok("获取成功", Some(records))
}

#[derive(Deserialize)]
pub struct SendMsg {
    r#type: String,
    content: String,
    from: u64,
    to: u64,
}

#[post("/send_msg")]
pub async fn send_msg(data: web::Json<SendMsg>) -> impl Responder {
    type Json = Response<()>;
    Record::add_record(data.from, data.to, &data.r#type, &data.content).await;
    Json::ok("发送成功", None)
}

#[derive(MultipartForm)]
pub struct Upload {
    pub user_id: Option<Text<u64>>,
    pub file: Tempfile
}

#[post("/upload")]
pub async fn upload(form: MultipartForm<Upload>) -> impl Responder {
    type Json = Response<String>;

    if form.user_id.is_none() || form.user_id.as_ref().unwrap().0 == 0 {
        return Json::err("id格式错误");
    }

    let user_id = Uuid::new();

    let file_ext = form.file.file_name.as_ref().unwrap().split(".").last().unwrap();
    let filename = format!("{}.{}", user_id, file_ext);
    fs::rename(form.file.file.path().to_str().unwrap(), format!("./static/{}", filename)).expect("can't rename file");

    Json::ok("上传成功", Some(filename))
}
