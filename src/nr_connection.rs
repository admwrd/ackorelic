use diesel::connection::{AnsiTransactionManager, SimpleConnection};
use diesel::deserialize::{Queryable, QueryableByName};
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
use diesel::result::{ConnectionResult, Error, QueryResult};
use diesel::sql_types::HasSqlType;

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
        //println(query)
        //let r = self.conn.execute(query)
        //print r()
        //r
        self.conn.execute(query)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: AsQuery,
        T::Query: QueryFragment<Pg> + QueryId,
        Pg: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Pg>,
    {
        self.conn.query_by_index(source)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: QueryFragment<Pg> + QueryId,
        U: QueryableByName<Pg>,
    {
        self.conn.query_by_name(source)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Pg> + QueryId,
    {
        self.conn.execute_returning_count(source)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        self.conn.transaction_manager()
    }
}
