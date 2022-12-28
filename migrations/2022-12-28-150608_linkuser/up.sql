-- Your SQL goes here
ALTER TABLE `xslink`.`link` 
ADD CONSTRAINT `fk_link_1`
  FOREIGN KEY (`brand_id`)
  REFERENCES `xslink`.`brand` (`id`)
  ON DELETE CASCADE
  ON UPDATE CASCADE;