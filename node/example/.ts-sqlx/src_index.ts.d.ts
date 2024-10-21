import type { SqlxString } from 'ts-sqlx';

declare module 'ts-sqlx' {
	export function sqlx(query: `select 
		p.id,
		p.title,
		p.content,
		p.created_at,
		p.updated_at
		from posts p`): SqlxString<[], {id: number, title: string, content: string, created_at: Date, updated_at: Date}>;

	export function sqlx(query: `select p.* from posts p where p.id = $1;`): SqlxString<[number], {id: number, title: string, content: string, updated_at: Date, created_at: Date}>;
}
