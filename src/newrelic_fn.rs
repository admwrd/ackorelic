use crate::nr_init::NR_APP;
use crate::transaction::Transaction;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use crate::segment::Segment;

 thread_local! {
    pub static TL_TRANSACTION: RefCell<Transaction> = RefCell::new(NR_APP
        .web_transaction("init")
        .expect("Could not start transaction"));
}

pub fn nr_start_web_transaction(name: &str) -> () {
    println!("Starting web transaction name : {}", name);
    let transaction = NR_APP
        .web_transaction(name)
        .expect("Could not start transaction");

    TL_TRANSACTION.with(|tr| {
        *tr.borrow_mut() = transaction;
    });
}

pub fn nr_end_transaction() {
    println!("Ending web transaction");
    TL_TRANSACTION.with(|tr| {
        tr.borrow_mut().end();
    });
}

pub fn nr_start_custom_segment(name: &str) -> () {
    println!("Starting custom segment name : {}", name);
    TL_TRANSACTION.with(|tr| {
        Segment::custom(&tr.borrow_mut(), name,"Custom");
    });
}