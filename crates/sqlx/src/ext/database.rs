#[macro_export]
macro_rules! impl_connection_with {
    ($database:ty, $connection:ty) => {
        impl ExecutorWith<$database> for $connection
        where
            for<'c> &'c mut <$database as Database>::Connection: Executor<'c, Database = $database>,
            for<'q> <$database as Database>::Arguments<'q>: IntoArguments<'q, $database>,
            for<'p> String: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> bool: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> i16: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> i32: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> i64: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> f32: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> f64: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> chrono::NaiveDate: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> chrono::NaiveDateTime: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
            for<'p> Vec<u8>: Encode<'p, $database> + Decode<'p, $database> + Type<$database>,
        {
            async fn execute_with_sql<'q>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<<$database as Database>::QueryResult, Error> {
                builder::query::<$database>(sql, args).execute(&mut *self).await
            }

            async fn list<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Vec<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, <$database as Database>::Row>
            {
                builder::query_as::<$database, O>(sql, args).fetch_all(&mut *self).await
            }

            async fn list_page<'q, O>(&mut self, sql: &'q str, param: PaginatedParam<'q>) -> Result<Paginated<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, <$database as Database>::Row>,
                u64: Type<$database>,
                for<'d> u64: Decode<'d, $database>,
                usize: ColumnIndex<<$database as Database>::Row>
            {
                let page = param.page.unwrap_or(1);
                let size = param.size.unwrap_or(10);
                let offset = (page - 1) as u64 * size;
                let query_sql = format!("{} limit {} offset {}", sql, size, offset);
                let params = param.params;
                // let conn = self.acquire().await?;
                let data = builder::query_as::<$database, O>(&query_sql, params)
                    .fetch_all(&mut *self).await?;
                let count_sql = format!("select count(*) from ({}) as count_query", sql);
                let total = builder::query_scalar::<$database, u64>(&count_sql, params)
                    .fetch_optional(&mut *self).await?;
                let total = total.unwrap_or(0);
                Ok(Paginated{ data, size, page, total })
            }

            async fn list_row<'q>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Vec<<$database as Database>::Row>, Error> {
                builder::query::<$database>(sql, args).fetch_all(&mut *self).await
            }

            async fn first<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Option<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, <$database as Database>::Row>
            {
                builder::query_as::<$database, O>(sql, args).fetch_optional(&mut *self).await
            }

            async fn first_row<'q>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Option<<$database as Database>::Row>, Error> {
                builder::query::<$database>(sql, args).fetch_optional(&mut *self).await
            }

            async fn scalar<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Option<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: Type<$database>,
                O: for<'d> Decode<'d, $database>,
                (O,): for<'o> FromRow<'o, <$database as Database>::Row>,
                usize: ColumnIndex<<$database as Database>::Row>
            {
                builder::query_scalar::<$database, O>(sql, args).fetch_optional(&mut *self).await
            }

            async fn execute_with_query<'q>(&mut self, query: Query<'q, $database, <$database as Database>::Arguments<'q>>) -> Result<<$database as Database>::QueryResult, Error> {
                query.execute(&mut *self).await
            }

            async fn list_with_query<'q, O>(&mut self, query: QueryAs<'q, $database, O, <$database as Database>::Arguments<'q>>) -> Result<Vec<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, <$database as Database>::Row>
            {
                query.fetch_all(&mut *self).await
            }

            async fn list_row_with_query<'q>(&mut self, query: Query<'q, $database, <$database as Database>::Arguments<'q>>) -> Result<Vec<<$database as Database>::Row>, Error> {
                query.fetch_all(&mut *self).await
            }

            async fn first_with_query<'q, O>(&mut self, query: QueryAs<'q, $database, O, <$database as Database>::Arguments<'q>>) -> Result<Option<O>, Error>
            where
                O: Send + Unpin + 'q,
                O: for<'o> FromRow<'o, <$database as Database>::Row>
            {
                query.fetch_optional(&mut *self).await
            }

            async fn first_row_with_query<'q>(&mut self, query: Query<'q, $database, <$database as Database>::Arguments<'q>>) -> Result<Option<<$database as Database>::Row>, Error> {
                query.fetch_optional(&mut *self).await
            }

            async fn scalar_with_query<'q, O>(&mut self, query: QueryScalar<'q, $database, O, <$database as Database>::Arguments<'q>>) -> Result<Option<O>, Error>
            where
                O: Send + Unpin + 'q,
                (O,): for<'o> FromRow<'o, <$database as Database>::Row>,
                usize: ColumnIndex<<$database as Database>::Row>,
            {
                query.fetch_optional(&mut *self).await
            }
        }
    };
}