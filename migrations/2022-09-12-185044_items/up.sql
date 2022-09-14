create table if not exists items (
    id serial primary key,
    name text not null,
    uid int not null,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

create trigger user_updated before insert or update on items
for each row execute procedure update_timestamp();