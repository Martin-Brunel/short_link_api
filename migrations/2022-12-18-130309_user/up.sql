-- Your SQL goes here
CREATE TABLE user (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `email` VARCHAR(100) NOT NULL,
    password VARCHAR(256) NOT NULL
);