pub mod user;
pub mod record;

use std::sync::Mutex;
use once_cell::sync::Lazy;

use rbatis::{Rbatis, Error};
use rbdc_mysql::driver::MysqlDriver;

pub static CONNECTION: Lazy<Mutex<Rbatis>> = Lazy::new(|| Mutex::new(Rbatis::new()));

pub fn init() -> Result<(), Error> {
    CONNECTION.lock().unwrap().init(MysqlDriver{},"mysql://root:root@localhost:3306/xky")
}
