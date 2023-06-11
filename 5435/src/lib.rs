
#![no_std]
use gstd::{prelude::*, msg, ActorId, Debug};

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum InputMessage {
    SendHelloTo(ActorId),
    SendHelloReply,
}

static mut GREETING: Option<String> = None;

#[no_mangle]
extern "C" fn init() {
    let init_message: String = msg::load().expect("can't load the incoming message");
    Debug!("Program initialized: {:?}", init_message);
    unsafe {
        GREETING = Some(init_message);
    }
}

#[no_mangle]
extern "C" fn handle() {
    msg::reply(String::from("Hello world"), 0).expect("error while trying to send reply");
}
