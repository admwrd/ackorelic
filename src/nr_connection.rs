use diesel::{
    connection::{AnsiTransactionManager, SimpleConnection},
    deserialize::{Queryable, QueryableByName},
    pg::PgQueryBuilder,
    pg::{Pg, PgConnection, TransactionBuilder},
    prelude::*,
    query_builder::QueryBuilder,
    query_builder::{AsQuery, QueryFragment, QueryId},
    result::{ConnectionResult, QueryResult},
    sql_types::HasSqlType,
};

use crate::{
    newrelic_fn::TL_TRANSACTION, nr_init::ENABLE_NEW_RELIC, utils, Datastore,
    DatastoreParamsBuilder,
};

//use diesel::query_builder::DebugQuery;

pub struct NRConnection {
    pub conn: PgConnection,
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
        // println!("NRConnection::establish database_url: {}", database_url);
        if *ENABLE_NEW_RELIC {
            let segment_params = DatastoreParamsBuilder::new(Datastore::Postgres)
                .collection("establish_connection")
                .build()
                .expect("Invalid datastore segment parameters");
            TL_TRANSACTION.with(|tr| {
                let value = tr.borrow_mut().datastore_segment(&segment_params, |_| {
                    let pg_conn = PgConnection::establish(database_url)?;
                    Ok(NRConnection { conn: pg_conn })
                });
                value
            })
        } else {
            let pg_conn = PgConnection::establish(database_url)?;
            Ok(NRConnection { conn: pg_conn })
        }
        //let pg_conn = PgConnection::establish(database_url)?;
        //Ok(NRConnection{conn: pg_conn})
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        // println!("NRConnection::execute query: {}",query);
        if *ENABLE_NEW_RELIC {
            let (operation, collection) = utils::parse_sql(&query);
            let segment_params = DatastoreParamsBuilder::new(Datastore::Postgres)
                .collection(&collection)
                .operation(&operation)
                .query(&query.replace("\"", ""))
                .build()
                .expect("Invalid datastore segment parameters");
            TL_TRANSACTION.with(|tr| {
                let value = tr
                    .borrow_mut()
                    .datastore_segment(&segment_params, |_| self.conn.execute(query));
                value
            })
        } else {
            self.conn.execute(query)
        }
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: AsQuery,
        T::Query: QueryFragment<Pg> + QueryId,
        Pg: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Pg>,
    {
        if *ENABLE_NEW_RELIC {
            let query = source.as_query();
            let query_str = diesel::debug_query(&query).to_string();
            let (operation, collection) = utils::parse_sql(&query_str);
            // println!("NRConnection::query_by_index :{}", query_str);

            let segment_params = DatastoreParamsBuilder::new(Datastore::Postgres)
                .collection(&collection)
                .operation(&operation)
                .query(&query_str.replace("\"", ""))
                .build()
                .expect("Invalid datastore segment parameters");

            TL_TRANSACTION.with(|tr| {
                let value = tr
                    .borrow_mut()
                    .datastore_segment(&segment_params, |_| self.conn.query_by_index(query));
                value
            })
        } else {
            self.conn.query_by_index(source)
        }
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: QueryFragment<Pg> + QueryId,
        U: QueryableByName<Pg>,
    {

        if *ENABLE_NEW_RELIC {
            let query = {
                let mut qb = PgQueryBuilder::default();
                source.to_sql(&mut qb)?;
                qb.finish()
            };
            // println!("NRConnection::query_by_name :: {}", query);
            let (operation, collection) = utils::parse_sql(&query);
            let segment_params = DatastoreParamsBuilder::new(Datastore::Postgres)
                .collection(&collection)
                .operation(&operation)
                .query(&query.replace("\"", ""))
                .build()
                .expect("Invalid datastore segment parameters");

            TL_TRANSACTION.with(|tr| {
                let value = tr
                    .borrow_mut()
                    .datastore_segment(&segment_params, |_| self.conn.query_by_name(source));
                value
            })
        } else {
            self.conn.query_by_name(source)
        }
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Pg> + QueryId,
    {
        if *ENABLE_NEW_RELIC {
            let query = {
                let mut qb = PgQueryBuilder::default();
                source.to_sql(&mut qb)?;
                qb.finish()
            };
            // println!("NRConnection::execute_returning_count: {}", query);
            let (operation, collection) = utils::parse_sql(&query);
            let segment_params = DatastoreParamsBuilder::new(Datastore::Postgres)
                .collection(&collection)
                .operation(&operation)
                .query(&query.replace("\"", ""))
                .build()
                .expect("Invalid datastore segment parameters");

            TL_TRANSACTION.with(|tr| {
                let value = tr.borrow_mut().datastore_segment(&segment_params, |_| {
                    self.conn.execute_returning_count(source)
                });
                value
            })
        } else {
            self.conn.execute_returning_count(source)
        }
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        //println!("NRConnection::transaction_manager");
        self.conn.transaction_manager()
    }
}

impl NRConnection {
    pub fn build_transaction(&self) -> TransactionBuilder {
        self.conn.build_transaction()
    }
}
