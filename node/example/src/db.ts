import Pool from 'pg-pool';
import type { SqlxString } from 'ts-sqlx';
import 'dotenv/config';

const databaseUrl = new URL(process.env.DATABASE_URL as string);

export const pool = new Pool({
	user: databaseUrl.username,
	password: databaseUrl.password,
	host: databaseUrl.hostname,
	port: Number(databaseUrl.port),
	database: databaseUrl.pathname.substring(1)
});

export function queryOptional<P extends unknown[], R = unknown>(
	query: SqlxString<P, R>,
	...params: P
): Promise<R | undefined> {
	return pool.query(query, params).then((result) => result.rows[0]);
}

export function queryOne<P extends unknown[], R = unknown>(
	query: SqlxString<P, R>,
	...params: P
): Promise<R> {
	return pool.query(query, params).then((result) => {
		if (result.rows.length === 0) {
			throw new Error('No rows found');
		}
		return result.rows[0];
	});
}

export function query<P extends unknown[], R = unknown>(
	query: SqlxString<P, R>,
	...params: P
): Promise<R[]> {
	return pool.query(query, params).then((result) => result.rows);
}
