--      User table
CREATE TABLE "user" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

    username VARCHAR(128) NOT NULL UNIQUE,

    pwd varchar(128), 
    pwd_salt uuid DEFAULT gen_random_uuid(),
    token_salt uuid DEFAULT gen_random_uuid()
);

--      Task table
CREATE TABLE task (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

    title VARCHAR(256) NOT NULL
);