use crate::nr_init::NR_APP;
use crate::transaction::Transaction;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use crate::acko_segment::Segment;
use std::thread::sleep;
use std::time::Duration;


//pub struct TLData {
//    transaction: Transaction,
//    segments: Vec<ffi::newrelic_segment_t>,
//}

 thread_local! {
    pub static TL_TRANSACTION: RefCell<Transaction> = RefCell::new(NR_APP
        .web_transaction("init")
        .expect("Could not start transaction"));

    //pub static TL_SEGMENT: RefCell<Segment> = RefCell::new(Segment::custom( TL_TRANSACTION.with(|tr| {*tr.borrow_mut()}), "init","Custom"));
}

pub fn nr_start_web_transaction(name: &str) -> () {
    println!("Starting web transaction name : {}", name);


    TL_TRANSACTION.with(|tr| {
        let transaction = NR_APP
        .web_transaction(name)
        .expect("Could not start transaction");
        *tr.borrow_mut() = transaction;
    });

}

pub fn nr_end_transaction() {
    println!("Ending web transaction");
    TL_TRANSACTION.with(|tr| {
        tr.borrow_mut().end();
    });
}

pub fn nr_start_custom_segment(name: &str) -> Segment {
    println!("Starting custom segment name : {}", name);

    let seg = TL_TRANSACTION.with(|tr| {
        let t = tr.borrow_mut();
        Segment::start_custom_segment(&t, name)
    });
    seg
}

pub fn nr_end_custom_segment(mut segment: Segment) {
    println!("Ending custom segment");
    TL_TRANSACTION.with(|tr| {
        let t = tr.borrow_mut();
        segment.end_segment(&t);
    });
}