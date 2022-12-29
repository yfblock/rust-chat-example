use actix_web::{ web, Responder, get, post };
use serde::Deserialize;
use crate::db::user::User;
use crate::response::Response;

#[derive(Debug, Deserialize)]
pub struct UserBasic {
    telephone: Option<String>,
    password: Option<String>
}

/// 登录接口
/// 
/// 判断账号是否存在 如果不存在直接注册 如果账号已经存在 则判断密码是否匹配登录
#[post("/login")]
pub async fn login(data: web::Json<UserBasic>) -> impl Responder {
    // 设置返回值类型
    type Json = Response<User>;

    // 判断参数是否合规
    if data.telephone.is_none() || data.password.is_none() {
        return Json::err("账号和密码不能为空");
    }

    // 读取列信息
    let telephone = data.telephone.as_ref().unwrap().trim();
    let password = data.password.as_ref().unwrap().trim();

    if telephone == "" || password == "" {
        return Json::err("请输入有效的账号");
    }

    match User::get_user_by_telephone(telephone).await {
        // 存在账号 尝试登录
        Some(mut user) => {
            // 判断密码是否相同
            if user.password.as_ref().unwrap() == password {
                user.password = None;
                Json::ok("登陆成功", Some(user))
            } else {
                Json::err("密码错误")
            }
        }
        // 不存在账号  所以注册
        None => {
            // 将账号信息 写入数据库
            let exec_res = User::add_user(data.telephone.as_ref().unwrap(), data.password.as_ref().unwrap()).await;
            log::info!("last insert id: {}", exec_res.last_insert_id);
            // 返回结果
            let user = User {
                id: exec_res.last_insert_id.as_u64().unwrap(),
                telephone: Some(telephone.to_string()),
                ..Default::default()
            };
            Json::ok("注册成功", Some(user))
        }
    }
}


#[get("/get_users")]
pub async fn get_users() -> impl Responder {
    type Json = Response<Vec<User>>;

    let users = User::get_users().await;

    Json::ok("获取成功", Some(users))
}
