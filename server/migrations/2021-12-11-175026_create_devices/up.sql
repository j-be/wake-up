create table if not exists devices
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR  NOT NULL,
    mac  CHAR(17) NOT NULL
);
