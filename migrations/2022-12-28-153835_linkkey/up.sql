-- Your SQL goes here
ALTER TABLE `xslink`.`link_view` 
ADD CONSTRAINT `fk_link_view_1`
  FOREIGN KEY (`link_id`)
  REFERENCES `xslink`.`link` (`id`)
  ON DELETE CASCADE
  ON UPDATE CASCADE;
