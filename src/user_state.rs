/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-24 12:39:21
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 15:38:19
 * @FilePath: /file_encrypt/src/user_state.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use crate::user_account::{Account, self};
use crate::{users_db_connect::{read_from_database, save_to_database}, users_db_operate::user_exist, pem};
use crypto::{digest::Digest, sha2::Sha256};

pub struct State {
    pub account: Account,
    pub priv_key: Vec<String>,
}

pub fn login() -> Option<Account> {
    let users: Vec<Account> = read_from_database();

    // 输入名称
    let mut username = String::new();
    println!("input username to login: ");
    std::io::stdin().read_line(&mut username).unwrap();

    // 判断是否存在
    while !user_exist(&users, &username.trim().to_string()) {
        println!("user do not exist, input again: ");
        username.clear();
        std::io::stdin().read_line(&mut username).unwrap();
    }

    // 判断是否为管理员
    if username.trim() == user_account::SEC6ET_RO07_N0M8 {
        println!("input password: ");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).unwrap();
        if password.trim() == user_account::SEC6ET_RO07_PA55 {
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

    // 获取对应用户名的密码
    let mut password = String::new();
    let mut salt = String::new();
    let mut account: Option<Account> = None;
    for user in &users {
        if user.username == username.trim().to_string() {
            password = user.password.clone();
            salt = user.salt.clone();
            account = Some(user.clone());
        }
    }

    // 输入密码
    let mut input = String::new();
    println!("input password: ");
    std::io::stdin().read_line(&mut input).unwrap();

    // 创建 SHA-256 哈希算法实例
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}{}", &input.trim(), salt));
    
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