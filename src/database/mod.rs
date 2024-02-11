use hashbrown::{hash_map::Entry, HashMap};
use once_cell::sync::Lazy;
use sqlx_core::{
  connection::Connection, database::Database, describe::Describe, executor::Executor, Result,
};
use std::sync::Mutex;

use crate::{runtime::block_on, ts::TSFieldType};

pub trait DatabaseExt: Database {
  fn field_type_for_id(id: &Self::TypeInfo) -> TSFieldType;
  fn describe_blocking(query: &str, database_url: &str) -> Result<Describe<Self>>;
}

pub struct CachingDescribeBlocking<DB: DatabaseExt> {
  connections: Lazy<Mutex<HashMap<String, DB::Connection>>>,
}

impl<DB: DatabaseExt> CachingDescribeBlocking<DB> {
  pub const fn new() -> Self {
    Self {
      connections: Lazy::new(|| Mutex::new(HashMap::new())),
    }
  }

  pub fn describe_blocking(&self, query: &str, database_url: &str) -> Result<Describe<DB>>
  where
    for<'a> &'a mut DB::Connection: Executor<'a, Database = DB>,
  {
    block_on(async {
      let mut cache = self
        .connections
        .lock()
        .expect("previous panic in describe call");

      let conn = match cache.entry(database_url.to_string()) {
        Entry::Occupied(hit) => hit.into_mut(),
        Entry::Vacant(miss) => miss.insert(DB::Connection::connect(&database_url).await?),
      };
      match conn.ping().await {
        Ok(_) => {}
        Err(e) => {
          cache.remove(database_url);
          return Err(e);
        }
      }

      conn.describe(query).await
    })
  }
}

macro_rules! impl_database_ts {
    (
        $database:path {
            $( $ty:ty => $ts:expr ),*
        }
    ) => {
        impl $crate::database::DatabaseExt for $database {
            fn field_type_for_id(info: &Self::TypeInfo) -> crate::ts::TSFieldType {
                match () {
                    $(
                        _ if <$ty as sqlx_core::types::Type<$database>>::type_info() == *info => $ts,
                    )*
                    $(
                        _ if <$ty as sqlx_core::types::Type<$database>>::compatible(info) => $ts,
                    )*
                    _ => $crate::ts::TSFieldType::Unknown,
                }
            }

            fn describe_blocking(
                query: &str,
                database_url: &str,
            ) -> sqlx_core::Result<sqlx_core::describe::Describe<Self>> {
                use $crate::database::CachingDescribeBlocking;
                static CACHE: CachingDescribeBlocking<$database> = CachingDescribeBlocking::new();
                CACHE.describe_blocking(query, database_url)
            }
        }
    }
}

#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

mod fake_sqlx {
  #[cfg(feature = "mysql")]
  pub use sqlx_mysql as mysql;

  #[cfg(feature = "postgres")]
  pub use sqlx_postgres as postgres;

  #[cfg(feature = "sqlite")]
  pub use sqlx_sqlite as sqlite;
}
