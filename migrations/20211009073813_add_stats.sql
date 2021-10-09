-- Add migration script here
create table user_stats
(
    user_id        integer not null
        constraint user_stats_pk primary key,
    offered_count  integer not null default 0,
    accepted_count integer not null default 0,
    declined_count integer not null default 0
);

create unique index user_stats_user_id_index on user_stats (user_id);