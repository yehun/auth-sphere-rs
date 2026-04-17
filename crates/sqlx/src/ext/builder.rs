use chrono::{NaiveDate, NaiveDateTime};
use sqlx::query::{Query, QueryAs, QueryScalar};
use sqlx::{ColumnIndex, Database, Decode, FromRow, IntoArguments, Type};
use crate::Param;

pub fn query<'q, DB>(sql: &'q str, params: Option<&[Param]>) -> Query<'q, DB, DB::Arguments<'q>>
where
	DB: Database,
	// for<'c> &'c mut DB::Connection: Executor<'c, Database=DB>,
	// for<'e> 'a + Executor<'e, Database = DB>,
	for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
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
	let mut query = sqlx::query(sql);
	if let Some(params) = params {
		for p in params {
			query = p.bind_to_query::<DB, DB::Row>(query);
		}
	}
	query
}

pub fn query_as<'q, DB, O>(sql: &'q str, params: Option<&[Param]>) -> QueryAs<'q, DB, O, DB::Arguments<'q>>
where
	DB: Database,
	// for<'c> &'c mut DB::Connection: Executor<'c, Database=DB>,
	for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
	O: Send + Unpin,
	O: for<'a> FromRow<'a, DB::Row>,
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
	let mut query = sqlx::query_as::<DB, O>(sql);
	if let Some(params) = params {
		for p in params {
			query = p.bind_to_query_as::<DB, O>(query);
		}
	}
	query
}

pub fn query_scalar<'q, DB, O>(sql: &'q str, params: Option<&[Param]>) -> QueryScalar<'q, DB, O, DB::Arguments<'q>>
where
	DB: Database,
	// for<'c> &'c mut <DB as Database>::Connection: Executor<'c, Database=DB>,
	for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
	usize: ColumnIndex<<DB as Database>::Row>,
	O: Send + Unpin,
	O: Type<DB>,
	O: for<'a> Decode<'a, DB>,
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
	let mut query = sqlx::query_scalar::<DB, O>(sql);
	if let Some(params) = params {
		for p in params {
			query = p.bind_to_query_scalar::<DB, O>(query)
		}
	}
	query
}
