use diesel::pg::PgConnection;
use diesel::connection::Connection;
use newrelic::nr_connection::NRConnection;
use newrelic::skill::Skill;
use newrelic::tables::users_skill::dsl::users_skill;

use diesel::prelude::*;

//thread_local! {
//    static transaction:
//}


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


    let results = users_skill
        .load::<Skill>(&nr_conn)
        .expect("Error loading skills");

    println!("Displaying {} skills", results.len());
    for skill in results {
        println!("id: {} name: {}", skill.id, skill.name);
    }
}