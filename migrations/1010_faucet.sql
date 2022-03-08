CREATE TABLE "users" (
	"id"	INTEGER,
	"discord_id"	TEXT NOT NULL UNIQUE,
	"claimed"	TEXT NOT NULL,
	"account_id"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id")
);