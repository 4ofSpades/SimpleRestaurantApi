CREATE TABLE "orders" (
  "id" SERIAL PRIMARY KEY,
  "table_id" int,
  "created_at" int,
  "items" varchar(255),
  "duration" int,
);


