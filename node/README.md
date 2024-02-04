# ts-sqlx

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue")](LICENSE-MIT)
[![npm (scoped)](https://img.shields.io/npm/v/ts-sqlx)](https://www.npmjs.com/package/ts-sqlx)

Typescript SQLx compile-time checked queries without a DSL.

```typescript
import { sqlx, type SqlxString } from 'ts-sqlx';
import Pool from 'pg-pool';

const pool = new Pool();

function query<P extends unknown[], R = unknown>(
	query: SqlxString<P, R>,
	...params: P
): Promise<R[]> {
	return pool.query(query, params).then((result) => result.rows);
}

const post = await query(sqlx('select p.* from posts p where p.id = $1;'), 1);
console.log(post);
```
