use crate::nr_init::NR_APP;
use crate::transaction::Transaction;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use crate::segment::Segment;

 thread_local! {
    pub static TL_TRANSACTION: RefCell<Transaction> = RefCell::new(NR_APP
        .web_transaction("init")
        .expect("Could not start transaction"));

    //pub static TL_SEGMENT: RefCell<Segment> = RefCell::new(Segment::custom( TL_TRANSACTION.with(|tr| {*tr.borrow_mut()}), "init","Custom"));
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
//    let seg = TL_TRANSACTION.with(|tr| {
//        Segment::custom(&tr.borrow_mut(), name,"Custom")
//    });
//    TL_SEGMENT.with(|segment| {
//        *segment.borrow_mut() = seg;
//    });
}

pub fn nr_end_custom_segment() -> () {
    println!("Ending custom segment");
//    TL_SEGMENT.with(|seg| {
//        seg.borrow_mut().end();
//    });
}