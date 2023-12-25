/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-23 15:24:35
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 13:55:25
 * @FilePath: /file_encrypt/user_account.rs
 * @Description: 定义用户账户以及设置恢复账号
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use serde::{Serialize, Deserialize};

pub const SEC6ET_RO07_N0M8: &str = "cumt@1909#cs_IS";
pub const SEC6ET_RO07_PA55: &str = "cumt&Since1909@IS";

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub level: String,
}