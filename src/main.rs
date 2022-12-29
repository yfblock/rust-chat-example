#[allow(invalid_type_param_default)]

pub mod response;
pub mod api;
pub mod db;

use std::fs;
use std::path::Path;

use actix::Actor;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let static_path = Path::new("./static");
    if !static_path.exists() {
        fs::create_dir("./static").expect("Can't create directory");
    }
    if let Ok(_) = db::init() {
        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .supports_credentials()
                        .max_age(3600),
                )
                .wrap(Logger::default())
                .service(actix_files::Files::new("/static", "./static"))
                .service(api::ws::index)
                .service(api::login::login)
                .service(api::login::get_users)
                .service(api::user::get_user_info)
                .service(api::user::upload_face)
                .service(api::user::add_friend)
                .service(api::user::get_friends)
                .service(api::record::get_user_record)
                .service(api::record::send_msg)
                .service(api::record::upload)
        })
        .bind(("0.0.0.0", 19000))?
        .run()
        .await
    } else {
        println!("数据库连接失败");
        Ok(())
    }
}


