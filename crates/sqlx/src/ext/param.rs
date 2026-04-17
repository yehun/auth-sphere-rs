use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::Database;
use sqlx::query::{Query, QueryAs, QueryScalar};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Param {
    String(String),
    Bool(bool),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Array(Vec<Self>),
}

impl Param {

    pub fn bind_to_query<'q, DB, O>(&self, query: Query<'q, DB, DB::Arguments<'q>>)
        -> Query<'q, DB, DB::Arguments<'q>>
    where
        DB: Database,
        String: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        bool: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i16: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i32: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i64: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        f32: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        f64: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        NaiveDate: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        NaiveDateTime: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        Vec<u8>: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
    {
        match self {
            Param::String(val) => query.bind(val.clone()),
            Param::Bool(val) => query.bind(*val),
            Param::Short(val) => query.bind(*val),
            Param::Int(val) => query.bind(*val),
            Param::Long(val) => query.bind(*val),
            Param::Float(val) => query.bind(*val),
            Param::Double(val) => query.bind(*val),
            Param::Date(val) => query.bind(*val),
            Param::DateTime(val) => query.bind(*val),
            Param::Array(array) => {
                let mut qry = query;
                if !array.is_empty() {
                    for val in array {
                        qry = val.bind_to_query::<DB, O>(qry);
                    }
                }
                qry
            }
        }
    }

    pub fn bind_to_query_as<'q, DB, O>(&self, query: QueryAs<'q, DB, O, DB::Arguments<'q>>)
        -> QueryAs<'q, DB, O, DB::Arguments<'q>>
    where
        DB: Database,
        String: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        bool: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i16: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i32: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i64: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        f32: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        f64: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        NaiveDate: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        NaiveDateTime: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        Vec<u8>: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
    {
        match self {
            Param::String(val) => query.bind(val.clone()),
            Param::Bool(val) => query.bind(*val),
            Param::Short(val) => query.bind(*val),
            Param::Int(val) => query.bind(*val),
            Param::Long(val) => query.bind(*val),
            Param::Float(val) => query.bind(*val),
            Param::Double(val) => query.bind(*val),
            Param::Date(val) => query.bind(*val),
            Param::DateTime(val) => query.bind(*val),
            Param::Array(array) => {
                let mut qry = query;
                if !array.is_empty() {
                    for val in array {
                        qry = val.bind_to_query_as::<DB, O>(qry);
                    }
                }
                qry
            }
        }
    }

    pub fn bind_to_query_scalar<'q, DB, O>(&self, query: QueryScalar<'q, DB, O, DB::Arguments<'q>>)
        -> QueryScalar<'q, DB, O, DB::Arguments<'q>>
    where
        DB: Database,
        String: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        bool: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i16: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i32: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        i64: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        f32: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        f64: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        NaiveDate: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        NaiveDateTime: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
        Vec<u8>: sqlx::Encode<'q, DB> + sqlx::Decode<'q, DB> + sqlx::Type<DB>,
    {
        match self {
            Param::String(val) => query.bind(val.clone()),
            Param::Bool(val) => query.bind(*val),
            Param::Short(val) => query.bind(*val),
            Param::Int(val) => query.bind(*val),
            Param::Long(val) => query.bind(*val),
            Param::Float(val) => query.bind(*val),
            Param::Double(val) => query.bind(*val),
            Param::Date(val) => query.bind(*val),
            Param::DateTime(val) => query.bind(*val),
            Param::Array(array) => {
                let mut qry = query;
                if !array.is_empty() {
                    for val in array {
                        qry = val.bind_to_query_scalar::<DB, O>(qry);
                    }
                }
                qry
            }
        }
    }
}
