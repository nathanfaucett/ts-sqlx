use sqlx_core::{column::Column, executor::Executor};
use std::fmt;
use url::Url;

use crate::database::DatabaseExt;

pub enum TSFieldType {
    String,
    Number,
    BigInt,
    Boolean,
    Object,
    Date,
    Null,
    Any,
    Array(Box<TSFieldType>),
    Tuple(Vec<TSFieldType>),
    Unknown,
    Never,
}

impl fmt::Display for TSFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TSFieldType::String => write!(f, "string"),
            TSFieldType::Number => write!(f, "number"),
            TSFieldType::BigInt => write!(f, "bigint"),
            TSFieldType::Boolean => write!(f, "boolean"),
            TSFieldType::Object => write!(f, "object"),
            TSFieldType::Date => write!(f, "Date"),
            TSFieldType::Null => write!(f, "null"),
            TSFieldType::Any => write!(f, "any"),
            TSFieldType::Array(t) => write!(f, "Array<{}>", t),
            TSFieldType::Tuple(v) => {
                write!(f, "[")?;
                for (i, t) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, "]")
            }
            TSFieldType::Unknown => write!(f, "unknown"),
            TSFieldType::Never => write!(f, "never"),
        }
    }
}

pub struct QueryToTSDriver {
    url_schemes: &'static [&'static str],
    to_ts_call: fn(&str, &str) -> sqlx_core::Result<TSCall>,
}

impl QueryToTSDriver {
    pub const fn new<DB: DatabaseExt>() -> Self
    where
        for<'a> &'a mut DB::Connection: Executor<'a, Database = DB>,
    {
        QueryToTSDriver {
            url_schemes: DB::URL_SCHEMES,
            to_ts_call: to_ts_call::<DB>,
        }
    }

    pub fn to_ts_call(&self, query: &str, database_url: &str) -> sqlx_core::Result<TSCall> {
        (self.to_ts_call)(query, database_url)
    }
}

pub const FOSS_DRIVERS: &[QueryToTSDriver] = &[
    QueryToTSDriver::new::<sqlx_mysql::MySql>(),
    QueryToTSDriver::new::<sqlx_postgres::Postgres>(),
    QueryToTSDriver::new::<sqlx_sqlite::Sqlite>(),
];

pub fn get_foss_driver_for_database_url(database_url: &str) -> anyhow::Result<&QueryToTSDriver> {
    let database_url_parsed: Url = database_url.parse()?;
    if let Some(driver) = FOSS_DRIVERS
        .into_iter()
        .find(|driver| driver.url_schemes.contains(&database_url_parsed.scheme()))
    {
        Ok(driver)
    } else {
        Err(anyhow::anyhow!("No driver found for {}", database_url))
    }
}

pub struct TSCall {
    pub query: String,
    pub params: Vec<TSFieldType>,
    pub result: Vec<(String, TSFieldType)>,
}

impl fmt::Display for TSCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "export function sqlx(query: `{}`): SqlxString<[{}], {{{}}}>;",
            self.query,
            self.params
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<_>>()
                .join(", "),
            self.result
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

pub fn to_ts_call<DB: DatabaseExt>(query: &str, database_url: &str) -> sqlx_core::Result<TSCall>
where
    for<'a> &'a mut DB::Connection: Executor<'a, Database = DB>,
{
    let describe = DB::describe_blocking(query, database_url)?;

    let mut result = Vec::new();
    for column in describe.columns() {
        result.push((
            column.name().to_owned(),
            DB::field_type_for_id(column.type_info()),
        ));
    }
    let mut params = Vec::new();
    match describe.parameters() {
        Some(sqlx_core::Either::Left(list)) => {
            for param in list {
                params.push(DB::field_type_for_id(param));
            }
        }
        Some(sqlx_core::Either::Right(_size)) => {}
        None => {}
    }

    Ok(TSCall {
        query: query.to_owned(),
        params,
        result,
    })
}

pub fn ts_calls_to_string(ts_calls: &[TSCall]) -> String {
    format!(
        "import type {{ SqlxString }} from 'ts-sqlx';\n\ndeclare module 'ts-sqlx' {{{}}}\n",
        ts_calls
            .iter()
            .map(|ts| format!("\n\t{}\n", ts))
            .collect::<Vec<String>>()
            .join("")
    )
}
