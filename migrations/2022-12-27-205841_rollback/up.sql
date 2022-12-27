-- Your SQL goes here
ALTER TABLE `short_links`.`link` 
DROP COLUMN `updated_at`,
DROP COLUMN `deleted_at`,
DROP COLUMN `created_at`,
DROP COLUMN `is_deleted`;