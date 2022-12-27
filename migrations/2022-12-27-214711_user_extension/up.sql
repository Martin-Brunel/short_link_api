-- Your SQL goes here
ALTER TABLE `xslink`.`user` 
ADD COLUMN `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP AFTER `roles`,
ADD COLUMN `is_deleted` TINYINT NOT NULL DEFAULT 0 AFTER `created_at`,
ADD COLUMN `deleted_at` DATETIME NULL AFTER `is_deleted`,
ADD COLUMN `updated_at` DATETIME NULL AFTER `deleted_at`;
