CREATE TABLE `user` (
  `uid` bigint unsigned NOT NULL AUTO_INCREMENT,
  `nickname` varchar(33) CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  `password` varchar(40) NOT NULL,
  `mail` varchar(33) CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  `url` varchar(60) CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  `last_login_time` bigint DEFAULT NULL,
  `create_time` bigint DEFAULT NULL,
  PRIMARY KEY (`uid`) USING BTREE,
  UNIQUE KEY `id` (`uid`)
) CHARSET=utf8mb3;