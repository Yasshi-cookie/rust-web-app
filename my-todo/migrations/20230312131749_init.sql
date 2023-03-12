CREATE TABLE `rust_web_app`.`todos`  (
  `id` int UNSIGNED NOT NULL AUTO_INCREMENT,
  `text` text NOT NULL,
  `completed` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`)
);
