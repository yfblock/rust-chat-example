use rbatis::rbdc::db::ExecResult;
use rbs::to_value;
use serde::{Serialize, Deserialize};

use super::CONNECTION;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Record {
    id: u64,
    r#type: String,
    content: String,
    from: u64,
    to: u64,
    to_user_face: Option<String>,
    from_user_face: Option<String>
}

impl Record {
    pub async fn get_records(from: u64, to: u64) -> Vec<Record> {
        CONNECTION.lock().unwrap()
        .fetch_decode("select * from record_view where (`from` = ? and `to` = ?) or (`from` = ? and `to` = ?)", 
        vec![to_value!(from), to_value!(to), to_value!(to), to_value!(from)])
        .await.unwrap()
    }

    pub async fn add_record(from: u64, to: u64, r#type: &str, content: &str) -> ExecResult {
        CONNECTION.lock().unwrap()
        .exec("insert into record (`from`,`to`,`type`,content) VALUES (?,?,?,?)", 
            vec![to_value!(from), to_value!(to), to_value!(r#type), to_value!(content)])
        .await.unwrap()
    }
}