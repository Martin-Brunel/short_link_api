-- Your SQL goes here
ALTER TABLE `xslink`.`link` 
ADD COLUMN `nb_clicks` INT NOT NULL DEFAULT 0 AFTER `label`;
