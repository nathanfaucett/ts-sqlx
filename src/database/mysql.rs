use super::fake_sqlx as sqlx;

impl_database_ts! {
    sqlx::mysql::MySql {
        u8 => crate::ts::TSFieldType::Number,
        u16 => crate::ts::TSFieldType::Number,
        u32 => crate::ts::TSFieldType::Number,
        u64 => crate::ts::TSFieldType::Number,
        i8 => crate::ts::TSFieldType::Number,
        i16 => crate::ts::TSFieldType::Number,
        i32 => crate::ts::TSFieldType::Number,
        i64 => crate::ts::TSFieldType::Number,
        f32 => crate::ts::TSFieldType::Number,
        f64 => crate::ts::TSFieldType::Number,

        String => crate::ts::TSFieldType::String,

        Vec<u8> => crate::ts::TSFieldType::Array(Box::new(crate::ts::TSFieldType::Number)),

        sqlx_core::types::chrono::NaiveTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::chrono::NaiveDate => crate::ts::TSFieldType::Date,

        sqlx_core::types::chrono::NaiveDateTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::chrono::DateTime<sqlx_core::types::chrono::Utc> => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::Time => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::Date => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::PrimitiveDateTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::time::OffsetDateTime => crate::ts::TSFieldType::Date,

        sqlx_core::types::BigDecimal => crate::ts::TSFieldType::Number,

        sqlx_core::types::Decimal => crate::ts::TSFieldType::Number,

        sqlx_core::types::JsonValue => crate::ts::TSFieldType::Object
    }
}
