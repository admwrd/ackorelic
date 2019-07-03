use diesel::pg::PgConnection;
use diesel::connection::Connection;
use newrelic::nr_connection::NRConnection;
use newrelic::skill::Skill;
use newrelic::tables::users_skill::dsl::users_skill;
use newrelic::tables::users_skill::dsl;

use diesel::prelude::*;
use diesel::sql_query;

use newrelic::transaction::Transaction;
use newrelic::newrelic_fn::{nr_start_web_transaction, nr_end_transaction};

//thread_local! {
//    static transaction: Transaction;
//}


//mod nr_connection;

use std::cell::RefCell;
use std::thread;

thread_local! {
    static FOO: RefCell<f32> = RefCell::new(1.0);
}

pub fn main(){
    nr_start_web_transaction("main tr");
    let database_url = "postgres://root@127.0.0.1/acko";
    //println!("in");
    let conn = PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url));
    //println!("out");
    let query = "select * from users_skill";
    let result = conn.execute(query).unwrap();
    //println!("pg result : {}", result);

    let nr_conn = NRConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url));
    let nr_result = nr_conn.execute(query).unwrap();
    //println!("nr result : {}", nr_result);


    let results = users_skill
        .filter(dsl::id.gt(20))
        .load::<Skill>(&nr_conn)
        .expect("Error loading skills");

    //println!("Displaying {} skills", results.len());
    for skill in results {
        //println!("id: {} name: {}", skill.id, skill.name);
    }



//    let result1 = sql_query(query)
//        .load::<Skill>(&nr_conn)
//        .expect("Error loading skills from sql query");
//
//    //println!("Displaying {} skills from sql query", results.len());
//    for skill in result1 {
//        //println!("id: {} name: {}", skill.id, skill.name);
//    }

    FOO.with(|foo| {
        // `foo` is of type `&RefCell<f64>`
        *foo.borrow_mut() = 3.0;
    });

    thread::spawn(move|| {
        // Note that static objects do not move (`FOO` is the same everywhere),
        // but the `foo` you get inside the closure will of course be different.
        FOO.with(|foo| {
            //println!("inner: {}", *foo.borrow());
        });
    }).join().unwrap();

    FOO.with(|foo| {
        //println!("main: {}", *foo.borrow());
    });

}