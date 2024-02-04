use super::fake_sqlx as sqlx;

impl_database_ts! {
    sqlx::sqlite::Sqlite {
        bool => crate::ts::TSFieldType::Boolean,
        i32 => crate::ts::TSFieldType::Number,
        i64 => crate::ts::TSFieldType::Number,
        f64 => crate::ts::TSFieldType::Number,
        String => crate::ts::TSFieldType::String,
        Vec<u8> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),

        sqlx_core::types::chrono::NaiveDate => crate::ts::TSFieldType::Date,

        sqlx_core::types::chrono::NaiveDateTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::chrono::DateTime<sqlx_core::types::chrono::Utc> => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::OffsetDateTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::PrimitiveDateTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::Date => crate::ts::TSFieldType::Date,

        sqlx_core::types::Uuid => crate::ts::TSFieldType::Date
    }
}
