CREATE TABLE "restaurants" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar
);

CREATE TABLE "waiters" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar,
  "restaurant_id" int
);

CREATE TABLE "tables" (
  "restaurant_id" int,
  "id" SERIAL PRIMARY KEY
);

CREATE TABLE "orders" (
  "id" SERIAL PRIMARY KEY,
  "table_id" int,
  "created_at" datetime DEFAULT (now())
);

CREATE TABLE "order_items" (
  "order_id" int,
  "item_id" int,
  "quantity" int DEFAULT 1,
  PRIMARY KEY ("order_id", "item_id")
);

CREATE TABLE "items" (
  "id" int PRIMARY KEY,
  "name" varchar,
  "duration" int
);

ALTER TABLE "waiters" ADD FOREIGN KEY ("restaurant_id") REFERENCES "restaurants" ("id");

ALTER TABLE "tables" ADD FOREIGN KEY ("restaurant_id") REFERENCES "restaurants" ("id");

ALTER TABLE "orders" ADD FOREIGN KEY ("table_id") REFERENCES "tables" ("id");

ALTER TABLE "order_items" ADD FOREIGN KEY ("order_id") REFERENCES "orders" ("id");

ALTER TABLE "order_items" ADD FOREIGN KEY ("item_id") REFERENCES "items" ("id");
