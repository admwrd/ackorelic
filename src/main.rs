use diesel::pg::PgConnection;
use diesel::connection::Connection;
use newrelic::nr_connection::NRConnection;
//extern crate diesel;
use newrelic::skill::Skill;
use newrelic::tables::users_skill::dsl::users_skill;
//use newrelic::*;

use diesel::prelude::*;
//extern crate newrelic;



//mod nr_connection;

pub fn main(){
    let database_url = "postgres://root@127.0.0.1/acko";
    let conn = PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url));

    let query = "select * from users_skill";
    let result = conn.execute(query).unwrap();
    println!("pg result : {}", result);

    let nr_conn = NRConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url));
    let nr_result = nr_conn.execute(query).unwrap();
    println!("nr result : {}", nr_result);

//    let s = String::from(query);
//
//    let query1 = nr_conn.query_by_name(&s)?;
//    println!("resp: {:?}", query1);
//
//


    let results = users_skill
        .load::<Skill>(&nr_conn)
        .expect("Error loading skills");

    println!("Displaying {} skills", results.len());
    for skill in results {
        println!("id: {} name: {}", skill.id, skill.name);
    }
}