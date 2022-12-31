-- Your SQL goes here
ALTER TABLE `xslink`.`user` 
ADD COLUMN `firstname` VARCHAR(100) NOT NULL DEFAULT '' AFTER `brand_id`,
ADD COLUMN `lastname` VARCHAR(100) NOT NULL DEFAULT '' AFTER `firstname`;
