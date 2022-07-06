-- Add up migration script here
create table users(
    id serial not null,
    username varchar(30) not null,
    password varchar(1024) not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null,

    primary key (id),
    unique(username)
);