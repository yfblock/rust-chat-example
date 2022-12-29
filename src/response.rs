use serde::{ Deserialize, Serialize };
use actix_web::web;


#[derive(Serialize, Deserialize)]
pub struct Response<T: Serialize = DefaultRes> {
    pub status: bool,
    pub msg: &'static str,
    pub data: Option<T>
}

#[derive(Serialize, Deserialize)]
pub struct DefaultRes();

impl<T: Serialize> Response<T> {
    pub fn ok(msg: &'static str, data: Option<T>) -> web::Json<Self> {
        web::Json(Response { 
            status: true, 
            msg, 
            data 
        })
    }

    pub fn err(msg: &'static str) -> web::Json<Response<T>> {
        web::Json(Response { 
            status: false, 
            msg, 
            data: Option::<T>::None
        })
    }
}
