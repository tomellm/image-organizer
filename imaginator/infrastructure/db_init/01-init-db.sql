create table media_data (
    uuid varchar(32) not null,
    original_name varchar(255) not null,
    current_name varchar(255) not null,
    extension varchar(10) not null,
    media_type smallint not null,
    datetime_created timestamp 
);
create table meta_data (
    uuid varchar(32) not null primary key,
    media_uuid varchar(32) not null,
    data_key varchar(255) not null,
    data_val text not null
);
create table xmp_data (
    uuid varchar(32) not null primary key,
    media_uuid varchar(32) not null,
    data_key varchar(255) not null,
    data_val text not null
);
