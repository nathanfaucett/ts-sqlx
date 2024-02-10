# ts-sqlx

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue")](LICENSE-MIT)
[![API](https://docs.rs/ts_sqlx/badge.svg)](https://docs.rs/ts-sqlx)
[![Crate](https://img.shields.io/crates/v/ts-sqlx.svg)](https://crates.io/crates/ts-sqlx)

Typescript SQLx compile-time checked queries without a DSL.

## Install

```bash
cargo install ts_sqlx
```

## Getting started

include path to generated declaration files in `tsconfig.json`

```json
{
  "include": [".ts-sqlx/*"]
}
```

create a .env file with `DATABASE_URL` set to your database url

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
```

run in watch mode with `ts-sqlx watch` in the root of your project, just once with `ts-sqlx run` or for help `ts-sqlx help`

## References

- [sqlx](https://github.com/launchbadge/sqlx)
- [sqlx-ts](https://github.com/JasonShin/sqlx-ts)
