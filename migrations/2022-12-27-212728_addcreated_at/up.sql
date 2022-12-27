-- Your SQL goes here
ALTER TABLE `short_links`.`link` 
ADD COLUMN `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP AFTER `user_id`;
