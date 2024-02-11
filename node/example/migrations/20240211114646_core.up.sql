CREATE FUNCTION "trigger_set_timestamp"()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TABLE "posts"(
	"id" SERIAL PRIMARY KEY,
	"title" VARCHAR(255) NOT NULL,
	"content" TEXT NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER "posts_set_timestamp" BEFORE UPDATE ON "posts" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

INSERT INTO "posts" ("title", "content") VALUES 
	('Hello world!', 'This is my first post!'),
	('I am older now.', 'This is my last post.');;