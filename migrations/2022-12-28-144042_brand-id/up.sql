-- Your SQL goes here
ALTER TABLE `xslink`.`link` 
ADD COLUMN `brand_id` INT NOT NULL AFTER `updated_at`;

