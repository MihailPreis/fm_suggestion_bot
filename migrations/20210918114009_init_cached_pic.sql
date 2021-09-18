-- Add migration script here
create table cached_pic
(
    id integer not null
        constraint offered_post_pk
            primary key autoincrement,
    image_name text not null,
    image_file_id text not null
);

create unique index cached_pic_id_uindex
    on cached_pic (id);

