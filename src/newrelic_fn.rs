use crate::nr_init::NR_APP;
use crate::transaction::Transaction;
use std::cell::RefCell;
use std::borrow::BorrowMut;

thread_local! {
    pub static TL_TRANSACTION: RefCell<Transaction> = RefCell::new(NR_APP
        .web_transaction("init")
        .expect("Could not start transaction"));
}

pub fn nr_start_web_transaction(name: &str) -> () {
    let transaction = NR_APP
        .web_transaction(name)
        .expect("Could not start transaction");

    TL_TRANSACTION.with(|tr| {
        *tr.borrow_mut() = transaction;
    });
}

pub fn nr_end_transaction() {
    TL_TRANSACTION.with(|tr| {
        tr.borrow_mut().end();
    });
}
