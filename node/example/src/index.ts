import { sqlx } from 'ts-sqlx';
import { query, queryOptional } from './db';
import './javascript';

interface Post {
	id: number;
	title: string;
	content: string;
	created_ts: Date;
	updated_ts: Date;
}

const post: Post | undefined = await queryOptional(
	sqlx('select p.* from posts p where p.id = $1;'),
	1
);
console.log(post?.id);

const posts: Post[] = await query(
	sqlx(`select 
    p.id,
    p.title,
    p.content,
    p.created_ts,
    p.updated_ts
    from posts p`)
);
console.log(posts);
