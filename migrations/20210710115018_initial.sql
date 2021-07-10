create table "user"
(
    id        serial       not null,
    public_id varchar(36)  not null,
    username  varchar(200) not null,
    constraint user_pk
        primary key (id)
);

create unique index user_public_id_uindex
    on "user" (public_id);

create unique index user_username_uindex
    on "user" (username);

create table session
(
    id         serial                   not null,
    public_id  varchar(36)              not null,
    user_id    integer                  not null,
    user_agent varchar(200)             not null,
    created    timestamp with time zone not null,
    constraint session_pk
        primary key (id),
    constraint session_user_id_fk
        foreign key (user_id) references "user"
            on delete cascade
);

create unique index session_public_id_uindex
    on session (public_id);

