-- Add migration script here
create table pic
(
    id integer not null
        constraint pic_pk
            primary key autoincrement,
    file_name text not null,
    for_accept boolean not null,
    data blob not null
);

create unique index pic_id_uindex
    on pic (id);

