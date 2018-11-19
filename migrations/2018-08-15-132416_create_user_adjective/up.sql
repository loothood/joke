create table user_adjective (
    id serial primary key,
    user_id int not null,
    adjective_id int not null,
    count int default 0
)