import { sqlx } from 'ts-sqlx';
import { queryOne } from './db';

export const random = await queryOne(
	sqlx(
		`select
		'{1,2,3}'::int[] as intarray,
		'{"a", "b", "c"}'::text[] as textarray,
		int4range(1, 3) as int4range,
		now() as now,
		'{"count": 1}'::jsonb as jsonb,
		'{now(), now()}'::timestamptz[] as timestamptzarray,
		daterange('2014/01/01', '2014/01/31', '[]') as daterange,
		interval '1 year' as interval
`,
		'another'
	)
);
console.log(random);
