-- Add migration script here
create table offered_post
(
    id integer not null
        constraint offered_post_pk
            primary key autoincrement,
    message_id integer not null,
    chat_id integer not null,
    admin_chat_id integer not null,
    admin_chat_message_id integer not null
);

create unique index offered_post_id_uindex
    on offered_post (id);

