use async_trait::async_trait;
use sqlx::Postgres as Db;
use sqlx::SqlitePool;
use sqlx::postgres::PgArguments as Arguments;
use sqlx::types::chrono;

use crate::db::error::DatabaseError;

type QA<'q, O> = sqlx::query::QueryAs<'q, Db, O, Arguments>;
type Q<'q> = sqlx::query::Query<'q, Db, Arguments>;

pub struct BaseTable {
    pub pool: SqlitePool,
}

impl BaseTable {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait TableBase {
    async fn create_table(&self) -> Result<(), DatabaseError>;
    async fn drop_table(&self) -> Result<(), DatabaseError>;
    async fn delete_all(&self) -> Result<(), DatabaseError>;
}

#[async_trait]
pub trait Table<T, ID>: TableBase {
    async fn select_all(&self) -> Result<Vec<T>, DatabaseError>;
    async fn insert(&self, model: &T) -> Result<ID, DatabaseError>;
    async fn select(&self, id: &ID) -> Result<Option<T>, DatabaseError>;
    async fn update(&self, model: &T) -> Result<(), DatabaseError>;
    async fn delete(&self, id: &ID) -> Result<(), DatabaseError>;
    async fn replace(&self, model: &T) -> Result<ID, DatabaseError>;
}

// Helper trait to handle binding parameters, especially for casting u64 to i64 for SQLite
pub trait BindParam<'q> {
    fn bind_param<O>(self, query: QA<'q, O>) -> QA<'q, O>;
    fn bind_param_q(self, query: Q<'q>) -> Q<'q>;
}

macro_rules! impl_bind_param {
    ($t:ty) => {
        impl<'q> BindParam<'q> for $t {
            fn bind_param<O>(self, query: QA<'q, O>) -> QA<'q, O> {
                query.bind(self)
            }
            fn bind_param_q(self, query: Q<'q>) -> Q<'q> {
                query.bind(self)
            }
        }
    };
}

// Implement for reference types that are passed to .bind()
impl_bind_param!(&'q i32);
impl_bind_param!(&'q i64);
impl_bind_param!(&'q String);
impl_bind_param!(&'q Option<String>);
impl_bind_param!(&'q chrono::DateTime<chrono::Utc>);

// For Json
impl<'q, T: serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync + 'static> BindParam<'q>
    for &'q sqlx::types::Json<T>
{
    fn bind_param<O>(self, query: QA<'q, O>) -> QA<'q, O> {
        query.bind(self)
    }
    fn bind_param_q(self, query: Q<'q>) -> Q<'q> {
        query.bind(self)
    }
}

// Special case for u64 (casting to i64)
impl<'q> BindParam<'q> for &'q u64 {
    fn bind_param<O>(self, query: QA<'q, O>) -> QA<'q, O> {
        query.bind(*self as i64)
    }
    fn bind_param_q(self, query: Q<'q>) -> Q<'q> {
        query.bind(*self as i64)
    }
}

macro_rules! impl_table {
    (
        $struct_name:ident,
        $model:ty,
        $table:expr,
        $pk:ident,
        $id_type:ty,
        $db_id_type:ty,
        $create_sql:expr,
        $cols:expr,
        $vals:expr,
        $update_set:expr,
        [ $( $field:ident ),+ ]
    ) => {
        pub struct $struct_name {
            base: BaseTable,
        }

        impl $struct_name {
            pub fn new(pool: SqlitePool) -> Self {
                Self {
                    base: BaseTable::new(pool),
                }
            }
        }

        #[async_trait]
        impl TableBase for $struct_name {
            async fn create_table(&self) -> Result<(), DatabaseError> {
                sqlx::query($create_sql)
                    .execute(&self.base.pool)
                    .await?;
                Ok(())
            }

            async fn drop_table(&self) -> Result<(), DatabaseError> {
                sqlx::query(concat!("DROP TABLE IF EXISTS ", $table))
                    .execute(&self.base.pool)
                    .await?;
                Ok(())
            }

            async fn delete_all(&self) -> Result<(), DatabaseError> {
                sqlx::query(concat!("DELETE FROM ", $table))
                    .execute(&self.base.pool)
                    .await?;
                Ok(())
            }
        }

        #[async_trait]
        impl Table<$model, $id_type> for $struct_name {
            async fn select_all(&self) -> Result<Vec<$model>, DatabaseError> {
                Ok(sqlx::query_as::<_, $model>(concat!("SELECT * FROM ", $table))
                    .fetch_all(&self.base.pool)
                    .await?)
            }

            async fn select(&self, id: &$id_type) -> Result<Option<$model>, DatabaseError> {
                let query = sqlx::query_as::<_, $model>(concat!("SELECT * FROM ", $table, " WHERE ", stringify!($pk), " = ?"));
                let query = BindParam::bind_param(id, query);
                Ok(
                    query
                        .fetch_optional(&self.base.pool)
                        .await?,
                )
            }

            async fn insert(&self, model: &$model) -> Result<$id_type, DatabaseError> {
                let mut query = sqlx::query_as(concat!(
                        "INSERT INTO ", $table, " (", $cols, ") VALUES (", $vals, ") RETURNING ", stringify!($pk)
                    ));

                $(
                    query = BindParam::bind_param(&model.$field, query);
                )+

                let row: ($db_id_type,) = query.fetch_one(&self.base.pool).await?;
                Ok(row.0 as $id_type)
            }

            async fn update(&self, model: &$model) -> Result<(), DatabaseError> {
                let mut query = sqlx::query(concat!(
                        "UPDATE ", $table, " SET ", $update_set, " WHERE ", stringify!($pk), " = ?"
                    ));

                $(
                    query = BindParam::bind_param_q(&model.$field, query);
                )+
                query = BindParam::bind_param_q(&model.$pk, query);

                query.execute(&self.base.pool).await?;
                Ok(())
            }

            async fn delete(&self, id: &$id_type) -> Result<(), DatabaseError> {
                let query = sqlx::query(concat!("DELETE FROM ", $table, " WHERE ", stringify!($pk), " = ?"));
                let query = BindParam::bind_param_q(id, query);
                query.execute(&self.base.pool).await?;
                Ok(())
            }

            async fn replace(&self, model: &$model) -> Result<$id_type, DatabaseError> {
                let mut query = sqlx::query_as(concat!(
                        "REPLACE INTO ", $table, " (", $cols, ") VALUES (", $vals, ") RETURNING ", stringify!($pk)
                    ));

                $(
                    query = BindParam::bind_param(&model.$field, query);
                )+

                let row: ($db_id_type,) = query.fetch_one(&self.base.pool).await?;
                Ok(row.0 as $id_type)
            }
        }
    };
}

// impl_table!(
//     FeedTable,
//     FeedModel,
//     "feeds",
//     id,
//     i32,
//     i32,
//     r#"CREATE TABLE IF NOT EXISTS feeds (
//         id INTEGER PRIMARY KEY AUTOINCREMENT,
//         name TEXT NOT NULL,
//         description TEXT DEFAULT NULL,
//         platform_id TEXT NOT NULL,
//         source_id TEXT NOT NULL,
//         items_id TEXT NOT NULL,
//         source_url TEXT NOT NULL,
//         cover_url TEXT DEFAULT NULL,
//         tags TEXT DEFAULT NULL,
//         UNIQUE(platform_id, source_id),
//         UNIQUE(source_url)
//     )"#,
//     "name, description, platform_id, source_id, items_id, source_url, cover_url, tags",
//     "?, ?, ?, ?, ?, ?, ?, ?",
//     "name = ?, description = ?, platform_id = ?, source_id = ?, items_id = ?, source_url = ?, cover_url = ?, tags = ?",
//     [
//         name,
//         description,
//         platform_id,
//         source_id,
//         items_id,
//         source_url,
//         cover_url,
//         tags
//     ]
// );
