-- Add migration script here
create table bans
(
    chat_id         integer not null
        constraint bans_pk primary key,
    user_name       text not null,
    date            text not null,
    is_ban          boolean not null default 0
);

create unique index bans_chat_id_index on bans (chat_id);