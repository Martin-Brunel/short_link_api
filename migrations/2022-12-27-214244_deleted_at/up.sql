-- Your SQL goes here
ALTER TABLE `xslink`.`link`
ADD COLUMN `deleted_at` DATETIME NULL AFTER `is_deleted`,
ADD COLUMN `updated_at` DATETIME NULL AFTER `deleted_at`;
