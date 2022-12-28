-- Your SQL goes here
ALTER TABLE `xslink`.`user` 
ADD CONSTRAINT `fk_user_1`
  FOREIGN KEY (`brand_id`)
  REFERENCES `xslink`.`brand` (`id`)
  ON DELETE CASCADE
  ON UPDATE CASCADE;