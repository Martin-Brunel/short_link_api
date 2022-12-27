-- Your SQL goes here
ALTER TABLE `short_links`.`link` 
ADD COLUMN `url` VARCHAR(100) NOT NULL AFTER `user_id`,
ADD COLUMN `code` VARCHAR(45) NOT NULL AFTER `url`,
ADD COLUMN `is_deleted` TINYINT NOT NULL DEFAULT 0 AFTER `code`,
ADD COLUMN `created_at` DATETIME NOT NULL DEFAULT now() AFTER `is_deleted`,
ADD COLUMN `deleted_at` DATETIME NULL AFTER `created_at`,
ADD COLUMN `updated_at` DATETIME NULL AFTER `deleted_at`,
CHANGE COLUMN `user_id` `user_id` INT NOT NULL ;