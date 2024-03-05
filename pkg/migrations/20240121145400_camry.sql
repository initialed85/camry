-- Create "stream" table
CREATE TABLE "public"."stream" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4(),
    "name" text NOT NULL,
    "url" text NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NULL,
    "deleted_at" timestamptz NULL,
    PRIMARY KEY ("id")
);
