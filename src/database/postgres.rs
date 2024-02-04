use super::fake_sqlx as sqlx;

impl_database_ts! {
    sqlx::postgres::Postgres {
        () => crate::ts::TSFieldType::Null,
        bool => crate::ts::TSFieldType::Boolean,
        String => crate::ts::TSFieldType::String,
        i8 => crate::ts::TSFieldType::Number,
        i16 => crate::ts::TSFieldType::Number,
        i32 => crate::ts::TSFieldType::Number,
        i64 => crate::ts::TSFieldType::Number,
        f32 => crate::ts::TSFieldType::Number,
        f64 => crate::ts::TSFieldType::Number,
        Vec<u8> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),

        sqlx::postgres::types::Oid => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgInterval => crate::ts::TSFieldType::Object,

        sqlx::postgres::types::PgMoney => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgLTree => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgLQuery => crate::ts::TSFieldType::String,

        sqlx_core::types::Uuid => crate::ts::TSFieldType::String,

        sqlx_core::types::chrono::NaiveTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::chrono::NaiveDate => crate::ts::TSFieldType::Date,

        sqlx_core::types::chrono::NaiveDateTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::chrono::DateTime<sqlx_core::types::chrono::Utc> => crate::ts::TSFieldType::Date,

        sqlx::postgres::types::PgTimeTz<sqlx_core::types::chrono::NaiveTime, sqlx_core::types::chrono::FixedOffset> => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::Time => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::Date => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::PrimitiveDateTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::OffsetDateTime => crate::ts::TSFieldType::Date,

        sqlx::postgres::types::PgTimeTz<sqlx_core::types::time::Time, sqlx_core::types::time::UtcOffset> => crate::ts::TSFieldType::Date,

        sqlx_core::types::BigDecimal => crate::ts::TSFieldType::Number,

        sqlx_core::types::Decimal => crate::ts::TSFieldType::Number,

        sqlx_core::types::ipnetwork::IpNetwork => crate::ts::TSFieldType::String,

        sqlx_core::types::mac_address::MacAddress => crate::ts::TSFieldType::String,

        sqlx_core::types::JsonValue => crate::ts::TSFieldType::Object,

        sqlx_core::types::BitVec => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Boolean)),

        Vec<bool> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Boolean)),
        Vec<String> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::String)),
        Vec<Vec<u8>> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),
        Vec<i8> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),
        Vec<i16> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),
        Vec<i32> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),
        Vec<i64> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),
        Vec<f32> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),
        Vec<f64> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),
        Vec<sqlx::postgres::types::Oid> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::String)),
        Vec<sqlx::postgres::types::PgMoney> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::String)),

        Vec<sqlx_core::types::Uuid> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::String)),

        Vec<sqlx_core::types::chrono::NaiveTime> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Date)),

        Vec<sqlx_core::types::chrono::NaiveDate> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Date)),

        Vec<sqlx_core::types::chrono::NaiveDateTime> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Date)),

        Vec<sqlx_core::types::chrono::DateTime<sqlx_core::types::chrono::Utc>> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Date)),

        Vec<sqlx_core::types::time::Time> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Date)),

        Vec<sqlx_core::types::time::Date> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Date)),

        Vec<sqlx_core::types::time::PrimitiveDateTime> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Date)),

        Vec<sqlx_core::types::time::OffsetDateTime> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Date)),

        Vec<sqlx_core::types::BigDecimal> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),

        Vec<sqlx_core::types::Decimal> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),

        Vec<sqlx_core::types::ipnetwork::IpNetwork> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::String)),

        Vec<sqlx_core::types::mac_address::MacAddress> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::String)),

        Vec<sqlx_core::types::JsonValue> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Object)),

        sqlx::postgres::types::PgRange<i32> => crate::ts::TSFieldType::String,
        sqlx::postgres::types::PgRange<i64> => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgRange<sqlx_core::types::BigDecimal> => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgRange<sqlx_core::types::Decimal> => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgRange<sqlx_core::types::chrono::NaiveDate> => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgRange<sqlx_core::types::chrono::NaiveDateTime> => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgRange<sqlx_core::types::chrono::DateTime<sqlx_core::types::chrono::Utc>> => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgRange<sqlx_core::types::time::Date> => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgRange<sqlx_core::types::time::PrimitiveDateTime> => crate::ts::TSFieldType::String,

        sqlx::postgres::types::PgRange<sqlx_core::types::time::OffsetDateTime> => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<i32>> => crate::ts::TSFieldType::String,
        Vec<sqlx::postgres::types::PgRange<i64>> => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<sqlx_core::types::BigDecimal>> => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<sqlx_core::types::Decimal>> => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<sqlx_core::types::chrono::NaiveDate>> => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<sqlx_core::types::chrono::NaiveDateTime>> => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<sqlx_core::types::chrono::DateTime<sqlx_core::types::chrono::Utc>>> => crate::ts::TSFieldType::String,

        &[sqlx::postgres::types::PgRange<sqlx_core::types::chrono::DateTime<sqlx_core::types::chrono::Utc>>] => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<sqlx_core::types::time::Date>> => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<sqlx_core::types::time::PrimitiveDateTime>> => crate::ts::TSFieldType::String,

        Vec<sqlx::postgres::types::PgRange<sqlx_core::types::time::OffsetDateTime>> => crate::ts::TSFieldType::String
    }
}
