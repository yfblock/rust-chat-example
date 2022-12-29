
use super::CONNECTION;
use rbatis::rbdc::db::ExecResult;
use rbs::to_value;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: u64,
    pub telephone: Option<String>,
    pub password: Option<String>,
    pub face: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FriendView {
    pub id: u64,
    pub telephone: String,
    pub to: u64,
    pub face: Option<String>
}

impl User {
    pub async fn get_users() -> Vec<User> {
        CONNECTION.lock().unwrap()
        .fetch_decode("select * from user", vec![])
        .await.unwrap()
    }
    
    pub async fn add_user(telephone: &str, password: &str) -> ExecResult {
        CONNECTION.lock().unwrap()
        .exec("insert into user (telephone, password) VALUES (?, ?)", vec![to_value!(telephone), to_value!(password)])
        .await.unwrap()
    }

    pub async fn get_user_by_id(id: u64) -> Option<User> {
        CONNECTION.lock().unwrap()
        .fetch_decode("select * from user where id = ?", vec![to_value!(id)])
        .await.unwrap()
    }
    
    pub async fn get_user_by_telephone(telephone: &str) -> Option<User> {
        CONNECTION.lock().unwrap()
        .fetch_decode("select * from user where telephone = ?", vec![to_value!(telephone)])
        .await.unwrap()
    }

    pub async fn update_face_by_id(id: u64, face: &str) {
        CONNECTION.lock().unwrap()
        .exec("update user set face = ? where id = ?", vec![to_value!(face), to_value!(id)])
        .await.unwrap();
    }

    pub async fn add_friend(from: u64, to: u64) {
        CONNECTION.lock().unwrap()
        .exec("insert into friend (`from`, `to`) VALUES (?, ?), (?,?)", vec![to_value!(from), to_value!(to), to_value!(to), to_value!(from)])
        .await.unwrap();
    }

    pub async fn get_friends(user_id: u64) -> Vec<FriendView> {
        CONNECTION.lock().unwrap()
        .fetch_decode("select * from friend_view where `from` = ?", vec![to_value!(user_id)])
        .await.unwrap()
    }

    pub async fn is_friend(from: u64, to: u64) -> bool {
        let data:Option<u64> = CONNECTION.lock().unwrap()
        .fetch_decode("select id from friend where `from`= ? and `to`= ?", vec![to_value!(from), to_value!(to)])
        .await.unwrap();

        data.is_some()
    }
}