-- Your SQL goes here
ALTER TABLE `user` 
ADD COLUMN `roles` JSON NOT NULL AFTER `password`;
