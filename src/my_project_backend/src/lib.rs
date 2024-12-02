use std::{borrow::BorrowMut, cell::RefCell};

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}

#[ic_cdk_macros::query]
fn get_msg() -> String{
    MSG.with(|msg| (*msg.borrow()).clone())
}

#[ic_cdk_macros::update]
fn set_msg(new_msg : String) -> () {
    MSG.with(|msg| *msg.borrow_mut() = new_msg)
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

ic_cdk::export_candid!();