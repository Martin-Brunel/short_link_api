-- Your SQL goes here
ALTER TABLE `xslink`.`link` 
ADD COLUMN `user_id` VARCHAR(45) NOT NULL AFTER `id`;
