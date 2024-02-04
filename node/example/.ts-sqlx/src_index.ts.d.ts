import type { SqlxString } from 'ts-sqlx';

declare module 'ts-sqlx' {
	export function sqlx(query: `select p.* from posts p where p.id = $1;`): SqlxString<[number], {id: number, title: string, content: string, updated_ts: Date, created_ts: Date}>;

	export function sqlx(query: `select 
    p.id,
    p.title,
    p.content,
    p.created_ts,
    p.updated_ts
    from posts p`): SqlxString<[], {id: number, title: string, content: string, created_ts: Date, updated_ts: Date}>;
}
