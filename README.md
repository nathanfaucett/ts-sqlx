# ts-sqlx

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue")](LICENSE-MIT)
[![API](https://docs.rs/ts_sqlx/badge.svg)](https://docs.rs/ts-sqlx)
[![Crate](https://img.shields.io/crates/v/ts-sqlx.svg)](https://crates.io/crates/ts-sqlx)

Typescript SQLx compile-time checked queries without a DSL.

## Getting started

install the ts-sqlx cli tool globally

```bash
cargo install ts_sqlx
```

in your project install the typescript definitions

```
npm install ts-sqlx -D
```

include the path to generated declaration files in your `tsconfig.json`

```json
{
  // default destination of declaration files
  "include": [".ts-sqlx/*"]
}
```

create a `.env` file with `DATABASE_URL` set to your database url

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
```

or create a `.ts-sqlx.json` file like

```json
{
  "databases": {
    "default": "postgres://postgres:postgres@localhost:5432/postgres",
    "another": "postgres://postgres:postgres@localhost:5432/postgres"
  },
  // defaults shown below, the rest of these are optional
  "src": ".",
  "dest": ".ts-sqlx",
  "extensions": ["ts", "tsx", "js", "jsx"],
  "ignore_patterns": ["*.d.ts"]
}
```

run in watch mode `ts-sqlx watch` in the root of your project, just once with `ts-sqlx run` or for help `ts-sqlx help`

## References

- [sqlx](https://github.com/launchbadge/sqlx)
- [sqlx-ts](https://github.com/JasonShin/sqlx-ts)
