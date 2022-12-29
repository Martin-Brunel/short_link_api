-- Your SQL goes here
ALTER TABLE `xslink`.`link` 
ADD COLUMN `label` VARCHAR(100) NOT NULL DEFAULT '' AFTER `brand_id`;