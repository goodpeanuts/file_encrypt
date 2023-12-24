/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-24 12:39:21
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 02:25:50
 * @FilePath: \file-cryption\src\state_user.rs
 * @Description: 图形化程序的用户状态
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use crate::user_account::{Account, self};
use crate::{users_db_connect::read_from_database, users_db_operate::user_exist, pem};
use crypto::{digest::Digest, sha2::Sha256};

pub struct State {
    pub account: Account,
    pub priv_key: Vec<String>,
}

pub fn login(input_username: &String, input_password: &String) -> Option<Account> {
    let users: Vec<Account> = read_from_database();

    // 判断是否存在
    if !user_exist(&users, &input_username.trim().to_string()) {
        println!("user not exist");
        return None;
    }

    // 判断是否为管理员
    if input_username.trim() == user_account::SEC6ET_RO07_N0M8 {
        if input_password.trim() == user_account::SEC6ET_RO07_PA55 {
            println!("login success");
            return Some(Account {
                username: "administator".to_string(),
                password: "*".to_string(),
                salt: "*".to_string(),
                level: "A".to_string(),
            });
        } else {
            println!("login failed");
            return None;
        }
    }

    let mut password = String::new();
    let mut salt = String::new();
    let mut account: Option<Account> = None;
    for user in &users {
        if user.username == input_username.trim().to_string() {
            password = user.password.clone();
            salt = user.salt.clone();
            account = Some(user.clone());
        }
    }


    // 创建 SHA-256 哈希算法实例
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}{}", &input_password.trim(), salt));
    
    if hasher.result_str() == password {
        println!("login success");
        return account;
    } else {
        println!("login failed");
    }
    None
}

pub fn get_priv_key(level: &str) -> Vec<String> {
    let mut keys = Vec::new();
    match level {
        "A" => {
            keys.push(pem::PRIV_KEY_ROOT.to_string());
        }
        "B" => {
            keys.push(pem::PRIV_KEY_A.to_string());
            keys.push(pem::PRIV_KEY_B.to_string());
        }
        "C" => {
            keys.push(pem::PRIV_KEY_C.to_string());
            keys.push(pem::PRIV_KEY_D.to_string());
        }
        "D" => {
            keys.push(pem::PRIV_KEY_B.to_string());
            keys.push(pem::PRIV_KEY_E.to_string());
        }
        "E" => {
            keys.push(pem::PRIV_KEY_D.to_string());
        }
        _ => {
            println!("level error");
        }
    }
    keys
}