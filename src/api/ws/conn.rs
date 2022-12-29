use actix_web_actors::ws;
use serde_json::Value;
use std::{sync::Mutex, collections::HashMap};
use actix::{WeakAddr, StreamHandler, Actor, AsyncContext, Addr, Message, Handler, Context};
use once_cell::sync::Lazy;

use crate::db::record::Record;

use super::messages::Command;

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageT(pub String);

pub struct ClientArray {
    clients: HashMap<u64, WeakAddr<WsConnection>>
}

impl Actor for ClientArray {
    type Context = Context<Self>;
}

impl ClientArray {
    pub fn new() -> Self {
        return ClientArray {
            clients: HashMap::new()
        }
    }

    pub fn add_clinet(&mut self, id: u64, addr: WeakAddr<WsConnection>) {
        // let mut index = usize::MAX;
        // for i in 0..self.clients.len() {
        //     if self.clients[i].upgrade().is_none() {
        //         index = i;
        //         break;
        //     }
        // }
        // if index == usize::MAX {
        //     self.clients.push(addr);
        // } else {
        //     self.clients[index] = addr;
        // }
        self.clients.insert(id, addr);
    }

    pub fn send_all(&self, op: impl Fn(Addr<WsConnection>)) {
        for client in self.clients.values() {
            if let Some(client) = client.upgrade() {
                // client.lock().unwrap().
                op(client);
            }
        }
    }

    pub fn send_to(&self, id: u64, op: impl Fn(Addr<WsConnection>)) {
        match self.clients.get(&id) {
            Some(client) => if let Some(client) = client.upgrade() {
                // client.lock().unwrap().
                op(client);
            },
            None => {}
        }
    }
}

pub static CLIENTS: Lazy<Mutex<ClientArray>> = Lazy::new(|| Mutex::new(ClientArray::new()));

/// Define HTTP actor
pub struct WsConnection {
    id: u64
}

impl WsConnection {
    pub fn new (id: u64) -> Self {
        Self {
            id
        }
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn started(&mut self, ctx: &mut Self::Context) {
        // 存储个人信息
        CLIENTS.lock().unwrap().add_clinet(self.id, ctx.address().downgrade());
        CLIENTS.lock().unwrap().send_all( |c| c.do_send(MessageT("Hello".to_string())));
        log::info!("{} connected", self.id);
    }

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let msg = text.trim();
                let command: Value = serde_json::from_str(msg).expect("can't analyze");
                log::info!("{:?}", command);

                let r#type = command["type"].as_str().unwrap();
                
                if r#type == "single" {
                    let target = command["target"].as_u64().unwrap();
                    CLIENTS.lock().unwrap().send_to(target, move |cr| {
                        cr.do_send(MessageT(msg.to_string()));
                    });
                } else {
                    // 进行对应的处理
                    CLIENTS.lock().unwrap().send_all(move |cr| {
                        cr.do_send(MessageT(msg.to_string()));
                    });
                }

                // 将聊天记录添加到数据库 目前在请求中已经完成了
                // let fut = async move {
                //     let add_record_future = Record::add_record(
                //         command["source"].as_u64().unwrap(), 
                //         command["target"].as_u64().unwrap(), 
                //         command["msgType"].as_str().unwrap(), 
                //         command["content"].as_str().unwrap()
                //     );
                //     let status = add_record_future.await;
            
                //     println!("child status was: {}", status);
                // };
                // ctx.spawn(actix::fut::wrap_future::<_, Self>(fut));
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            // 关闭协议
            Ok(ws::Message::Close(close)) => ctx.close(close),
            _ => (),
        };
    }
}

impl Handler<MessageT> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: MessageT, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}