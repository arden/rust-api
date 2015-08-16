/*
 Navicat Premium Data Transfer

 Source Server         : localhost
 Source Server Type    : MySQL
 Source Server Version : 50624
 Source Host           : localhost
 Source Database       : tujiao

 Target Server Type    : MySQL
 Target Server Version : 50624
 File Encoding         : utf-8

 Date: 07/06/2015 11:47:01 AM
*/

SET NAMES utf8;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
--  用户
-- ----------------------------
DROP TABLE IF EXISTS `t_users`;
CREATE TABLE `t_users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(50) NOT NULL COMMENT '用户名',
  `username` varchar(50) NOT NULL COMMENT '用户昵称',
  `mobile` varchar(50) NOT NULL COMMENT '手机号',
  `email` varchar(20) DEFAULT NULL COMMENT '邮箱',
  `password` varchar(50) NOT NULL COMMENT '手机号',
  `uuid` varchar(128) NOT NULL COMMENT '客户端唯一标识号',
  `token` varchar(128) DEFAULT NULL COMMENT '推送的令牌',
  `sex` int(2) DEFAULT NULL COMMENT '性别',
  `source` int(2) DEFAULT null COMMENT '用户注册来源(0->iPhone, 1->iPad, 2->Android)',
  `from_source` int(11) DEFAULT 0 COMMENT '通过什么方式注册的',
  `avatar` varchar(128) DEFAULT NULL COMMENT '头像',
  `lng` float DEFAULT '0.0' COMMENT '经度',
  `lat` float DEFAULT '0.0' COMMENT '纬度',
  `created_at` timestamp DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `status` int(2) DEFAULT 1 COMMENT '状态',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uuid` (`uuid`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8;
