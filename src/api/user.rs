use std::fs;

use actix_easy_multipart::MultipartForm;
use actix_easy_multipart::tempfile::Tempfile;
use actix_easy_multipart::text::Text;
use actix_web::{Responder, post, web, get};
use serde::Deserialize;


use crate::db::user::{User, FriendView};
use crate::response::{Response, DefaultRes};

#[derive(Deserialize)]
pub struct UserId {
    pub user_id: Option<u64>
}

#[get("/get_user_info")]
pub async fn get_user_info(data: web::Query<UserId>) -> impl Responder {
    type Json = Response<User>;

    if data.user_id.is_none() || data.user_id.unwrap() == 0 {
        return Json::err("id格式错误");
    }

    match User::get_user_by_id(data.user_id.unwrap()).await {
        Some(user) => {
            Json::ok("获取成功", Some(user))
        }
        None => Json::err("用户不存在")
    }
}


#[derive(MultipartForm)]
pub struct UploadFace {
    pub group_id: Option<Text<String>>,
    pub user_id: Option<Text<u64>>,
    pub file: Tempfile
}

#[post("/upload_face")]
pub async fn upload_face(form: MultipartForm<UploadFace>) -> impl Responder {
    type Json = Response<String>;

    if form.user_id.is_none() || form.user_id.as_ref().unwrap().0 == 0 {
        return Json::err("id格式错误");
    }

    let user_id = form.user_id.as_ref().unwrap().0;

    let file_ext = form.file.file_name.as_ref().unwrap().split(".").last().unwrap();
    fs::rename(form.file.file.path().to_str().unwrap(), format!("./static/{}.{}", user_id, file_ext)).expect("can't rename file");

    let filename = format!("{}.{}", user_id, file_ext);
    User::update_face_by_id(user_id, &filename).await;

    Json::ok("上传成功", Some(filename))
}


#[derive(Deserialize)]
pub struct AddFriend {
    pub user_id: u64,
    pub telephone: String
}

#[post("/add_friend")]
pub async fn add_friend(data: web::Json<AddFriend>) -> impl Responder {
    type Json = Response<DefaultRes>;

    // 判断是否存在用户
    let user = User::get_user_by_telephone(&data.telephone).await;
    if user.is_none() {
        return Json::err("用户不存在");
    }

    let user = user.unwrap();

    // 判断好友关系是否存在
    if User::is_friend(data.user_id, user.id).await {
        return Json::err("已经是好友了");
    }

    // 添加好友
    User::add_friend(data.user_id, user.id).await;

    Json::ok("添加成功", None)
}

#[post("/get_friends")]
pub async fn get_friends(data: web::Json<UserId>) -> impl Responder {
    type Json = Response<Vec<FriendView>>;

    if data.user_id.is_none() || data.user_id.unwrap() == 0 {
        return Json::err("id格式错误");
    }

    let friends = User::get_friends(data.user_id.unwrap()).await;
    
    Json::ok("获取成功", Some(friends))
}