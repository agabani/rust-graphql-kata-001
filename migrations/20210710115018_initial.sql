create table "user"
(
    id        bigserial    not null,
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
    id         bigserial                not null,
    public_id  varchar(36)              not null,
    user_id    bigint                   not null,
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

create table forum
(
    id                 bigserial                not null,
    public_id          varchar(36)              not null,
    created            timestamp with time zone not null,
    created_by_user_id bigint                   not null,
    name               varchar(50)              not null,
    constraint forum_pk
        primary key (id),
    constraint forum_user_id_fk
        foreign key (created_by_user_id) references "user"
);

create unique index forum_public_id_uindex
    on forum (public_id);

create table thread
(
    id                 bigserial                not null,
    public_id          varchar(36)              not null,
    created            timestamp with time zone not null,
    created_by_user_id bigint,
    name               varchar(50)              not null,
    forum_id           bigint                   not null,
    constraint thread_pk
        primary key (id),
    constraint thread_user_id_fk
        foreign key (created_by_user_id) references "user",
    constraint thread_forum_id_fk
        foreign key (forum_id) references forum
);

create unique index thread_public_id_uindex
    on thread (public_id);

create table reply
(
    id                 bigserial                not null,
    public_id          varchar(36),
    created            timestamp with time zone not null,
    created_by_user_id bigint                   not null,
    thread_id          bigint                   not null,
    text               text                     not null,
    constraint reply_pk
        primary key (id),
    constraint reply_user_id_fk
        foreign key (created_by_user_id) references "user",
    constraint reply_thread_id_fk
        foreign key (thread_id) references thread
);

create unique index reply_public_id_uindex
    on reply (public_id);

