import { sqlx } from 'ts-sqlx';
import { query, queryOne, queryOptional } from './db';
import './javascript';

interface Post {
	id: number;
	title: string;
	content: string;
	created_at: Date;
	updated_at: Date;
}

const post: Post | undefined = await queryOptional(
	sqlx('select p.* from posts p where p.id = $1;'),
	1
);
console.log(post);

const posts: Post[] = await query(
	sqlx(`select 
		p.id,
		p.title,
		p.content,
		p.created_at,
		p.updated_at
		from posts p`)
);
console.log(posts);

const anotherPost: Post = await queryOne(sqlx('select p.* from posts p where p.id = $1;'), 2);
console.log(anotherPost);
