-- Your SQL goes here
ALTER TABLE `xslink`.`user` 
ADD COLUMN `brand_id` INT NOT NULL AFTER `updated_at`;