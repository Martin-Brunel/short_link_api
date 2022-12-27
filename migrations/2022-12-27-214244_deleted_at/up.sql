-- Your SQL goes here
ALTER TABLE `short_links`.`link`
ADD COLUMN `deleted_at` DATETIME NULL AFTER `is_deleted`,
ADD COLUMN `updated_at` DATETIME NULL AFTER `deleted_at`;
