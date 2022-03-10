-- Add migration script here

create or replace function update_timestamp() returns trigger as $$
begin
    new.updated = now();
    return new;
end;
$$ language 'plpgsql';

create table if not exists packages (
    id serial primary key,
    name text not null,
    description text not null unique,
    downloads_count integer not null default 0,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

create trigger package_updated before insert or update on packages
for each row execute procedure update_timestamp();
