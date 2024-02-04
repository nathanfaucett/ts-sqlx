export type SqlxString<P, R> = string & { _type: { params: P; result: R } };

export type SqlxParams<T> = T extends SqlxString<infer P, infer _> ? P : never;
export type SqlxResult<T> = T extends SqlxString<infer _, infer R> ? R : never;

export function sqlx<P extends unknown[] = unknown[], R = unknown>(
	query: string
): SqlxString<P, R> {
	return query as SqlxString<P, R>;
}
