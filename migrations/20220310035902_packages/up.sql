-- Add migration script here

create table if not exists packages (
    id serial primary key,
    name text not null,
    description text not null unique,
    downloads_count integer not null default 0
);