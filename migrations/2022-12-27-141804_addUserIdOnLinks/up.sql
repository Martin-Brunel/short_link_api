-- Your SQL goes here
ALTER TABLE `short_links`.`link` 
ADD COLUMN `user_id` VARCHAR(45) NOT NULL AFTER `id`;
