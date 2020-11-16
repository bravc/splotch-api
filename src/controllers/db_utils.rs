use crate::diesel::{result::Error, RunQueryDsl};
use crate::Pool;
use actix_web::web;
use diesel::query_builder::QueryId;
use diesel::Queryable;
use diesel::{
    associations::HasTable, dsl::Find, query_builder::AsQuery, query_builder::InsertStatement,
    query_builder::QueryFragment, query_dsl::methods::FindDsl, query_dsl::LoadQuery,
    types::HasSqlType, Connection, PgConnection, Table,
};

pub fn insert_into_table<T, I, M>(pool: web::Data<Pool>, table: T, records: I) -> Result<M, Error>
where
    T: Table,
    I: diesel::Insertable<T>,
    InsertStatement<T, I::Values>: LoadQuery<PgConnection, M>,
{
    let conn = pool.get().unwrap();
    diesel::insert_into(table).values(records).get_result(&conn)
}

pub fn get_by_id<T, Pk, M>(pool: web::Data<Pool>, table: T, pk: Pk) -> Result<M, Error>
where
    T: FindDsl<Pk>,
    Find<T, Pk>: LoadQuery<PgConnection, M>,
{
    let conn = pool.get().unwrap();
    table.find(pk).get_result(&conn)
}

pub fn get_all<DB, T, I, Conn>(conn: &Conn, table: T) -> Result<Vec<I>, diesel::result::Error>
where
    Conn: Connection<Backend = DB>,
    DB: diesel::backend::Backend + HasSqlType<T::SqlType>,
    T: Table + AsQuery,
    T::Query: QueryFragment<DB> + QueryId,
    I: HasTable<Table = T> + Queryable<T::SqlType, DB>,
{
    table.load(conn)
}
