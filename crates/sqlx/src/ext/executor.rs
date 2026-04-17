#[macro_export]
macro_rules! impl_executor_with {
    ($executor:ident) => {
        #[allow(unused_mut)]
        impl<DB> ExecutorWith<DB> for $executor<DB>
        where
            DB: Database,
            for<'c> &'c mut <DB as Database>::Connection: Executor<'c, Database = DB>,
            for<'q> DB::Arguments<'q>: IntoArguments<'q, DB>,
            for<'p> String: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> bool: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> i16: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> i32: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> i64: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> f32: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> f64: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> NaiveDate: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> NaiveDateTime: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
            for<'p> Vec<u8>: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
        {
            async fn execute_with_sql<'q>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<DB::QueryResult, Error> {
                let mut conn = self.acquire().await?;
                builder::query::<DB>(sql, args).execute(&mut *conn).await
            }

            async fn list<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Vec<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, DB::Row>
            {
                let mut conn = self.acquire().await?;
                builder::query_as::<DB, O>(sql, args).fetch_all(&mut *conn).await
            }

            async fn list_page<'q, O>(&mut self, sql: &'q str, param: PaginatedParam<'q>) -> Result<Paginated<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, DB::Row>,
                u64: Type<DB>,
                for<'d> u64: Decode<'d, DB>,
                usize: ColumnIndex<<DB as Database>::Row>
            {
                let page = param.page.unwrap_or(1);
                let size = param.size.unwrap_or(10);
                let offset = (page - 1) as u64 * size;
                let query_sql = format!("{} limit {} offset {}", sql, size, offset);
                let params = param.params;
                let mut conn = self.acquire().await?;
                // let data = self.list::<O>(&query_sql, params).await?;
                let data = builder::query_as::<DB, O>(&query_sql, params)
                    .fetch_all(&mut *conn).await?;
                let count_sql = format!("select count(*) from ({}) as count_query", sql);
                // let total = self.pool.acquire().await?.scalar::<u64>(&count_sql, params).await?;
                let total = builder::query_scalar::<DB, u64>(&count_sql, params)
                    .fetch_optional(&mut *conn).await?;
                let total = total.unwrap_or(0);
                Ok(Paginated{ data, size, page, total })
            }

            async fn list_row<'q>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Vec<DB::Row>, Error> {
                let mut conn = self.acquire().await?;
                builder::query::<DB>(sql, args).fetch_all(&mut *conn).await
            }

            async fn first<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Option<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, DB::Row>
            {
                let mut conn = self.acquire().await?;
                builder::query_as::<DB, O>(sql, args).fetch_optional(&mut *conn).await
            }

            async fn first_row<'q>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Option<DB::Row>, Error> {
                let mut conn = self.acquire().await?;
                builder::query::<DB>(sql, args).fetch_optional(&mut *conn).await
            }

            async fn scalar<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Option<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: Type<DB>,
                O: for<'d> Decode<'d, DB>,
                (O,): for<'o> FromRow<'o, DB::Row>,
                usize: ColumnIndex<DB::Row>
            {
                let mut conn = self.acquire().await?;
                builder::query_scalar::<DB, O>(sql, args).fetch_optional(&mut *conn).await
            }

            async fn execute_with_query<'q>(&mut self, query: Query<'q, DB, DB::Arguments<'q>>) -> Result<DB::QueryResult, Error> {
                let mut conn = self.acquire().await?;
                query.execute(&mut *conn).await
            }

            async fn list_with_query<'q, O>(&mut self, query: QueryAs<'q, DB, O, DB::Arguments<'q>>) -> Result<Vec<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, DB::Row>
            {
                let mut conn = self.acquire().await?;
                query.fetch_all(&mut *conn).await
            }

            async fn list_row_with_query<'q>(&mut self, query: Query<'q, DB, DB::Arguments<'q>>) -> Result<Vec<DB::Row>, Error> {
                let mut conn = self.acquire().await?;
                query.fetch_all(&mut *conn).await
            }

            async fn first_with_query<'q, O>(&mut self, query: QueryAs<'q, DB, O, DB::Arguments<'q>>) -> Result<Option<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, DB::Row>,
            {
                let mut conn = self.acquire().await?;
                query.fetch_optional(&mut *conn).await
            }

            async fn first_row_with_query<'q>(&mut self, query: Query<'q, DB, DB::Arguments<'q>>) -> Result<Option<DB::Row>, Error> {
                let mut conn = self.acquire().await?;
                query.fetch_optional(&mut *conn).await
            }

            async fn scalar_with_query<'q, O>(&mut self, query: QueryScalar<'q, DB, O, DB::Arguments<'q>>) -> Result<Option<O>, Error>
            where
                O: Send + Unpin + 'q,
                (O,): for<'o> FromRow<'o, DB::Row>,
                usize: ColumnIndex<DB::Row>,
            {
                let mut conn = self.acquire().await?;
                query.fetch_optional(&mut *conn).await
            }
        }
    };
}