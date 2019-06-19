use diesel::connection::{AnsiTransactionManager, SimpleConnection};
use diesel::deserialize::{Queryable, QueryableByName};
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
use diesel::result::{ConnectionResult, QueryResult, Error};
use diesel::sql_types::HasSqlType;
//use diesel::query_builder::DebugQuery;

use std::{env, thread, time::Duration};

use crate::{App, Datastore, DatastoreParamsBuilder};


pub struct NRConnection {
    conn: PgConnection,
}
impl SimpleConnection for NRConnection {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        self.conn.batch_execute(query)
    }
}

impl Connection for NRConnection {
    type Backend = Pg;
    type TransactionManager = AnsiTransactionManager;

    fn establish(database_url: &str) -> ConnectionResult<NRConnection> {
        let pg_conn = PgConnection::establish(database_url)?;
        Ok(NRConnection{conn: pg_conn})
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        println!("NRConnection::execute query: {}",query);
        let result = self.conn.execute(query);
        //println!("NRConnection::execute query: {} result: {} ", query, result.unwrap());
        result
        //self.conn.execute(query)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: AsQuery,
        T::Query: QueryFragment<Pg> + QueryId,
        Pg: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Pg>,
    {
        let t_ref = source;
        let qu = t_ref.as_query();
        let q  = diesel::debug_query(&qu);
        let q_str = q.to_string();

        let license_key =
        env::var("NEW_RELIC_LICENSE_KEY").unwrap_or_else(|_| "example-license-key".to_string());
        let app = App::new("my app1", &license_key).expect("Could not create app");

        let transaction = app
        .web_transaction("Transaction name")
        .expect("Could not start transaction");

        let segment_params = DatastoreParamsBuilder::new(Datastore::Postgres)
        .collection("users_skill")
        .operation("select")
        .query(&q_str).build()
        .expect("Invalid datastore segment parameters");

        let value = transaction.datastore_segment(&segment_params, |_| {
            //self.conn.query_by_index(source)
            5
        });

        println!("NRConnection::query_by_index :{}", q.to_string());
        //println!("{}", diesel::debug_query(&t_ref.as_query()).to_string());

        //let t = self.conn.query_by_index(source);
        Err(Error::AlreadyInTransaction)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: QueryFragment<Pg> + QueryId,
        U: QueryableByName<Pg>,
    {
        println!("NRConnection::query_by_name");
        //let q = diesel::debug_query(&source.);
        //println!("NRConnection::query_by_index :{}", q.to_string());
        self.conn.query_by_name(source)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Pg> + QueryId,
    {
        println!("NRConnection::execute_returning_count");
        self.conn.execute_returning_count(source)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        self.conn.transaction_manager()
    }
}
