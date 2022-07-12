-- Add up migration script here
create table games
(
    id         serial                              not null,
    name       varchar(50)                         not null,
    link       varchar(200)                        null,
    author     varchar(50)                         not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null,

    primary key (id),
    unique (name)
);

create type attachment_type as enum ('screenshot', 'logo');

create table game_attachment
(
    id      serial          not null,
    type    attachment_type not null,
    path    varchar(500),
    game_id integer         not null,

    foreign key (game_id) REFERENCES games (id),
    unique (id)
);