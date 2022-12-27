-- Your SQL goes here
ALTER TABLE `short_links`.`link` 
ADD COLUMN `is_deleted` TINYINT NOT NULL DEFAULT 0 AFTER `created_at`;
