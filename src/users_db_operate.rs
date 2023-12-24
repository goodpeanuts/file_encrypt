/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-23 13:18:29
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 03:43:57
 * @FilePath: \file-cryption\src\users_db_operate.rs
 * @Description: 用户数据库管理
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::{
    user_account::{Account, SEC6ET_RO07_N0M8},
    users_db_connect::{read_from_database, save_to_database},
};
use crypto::{digest::Digest, sha2::Sha256};

pub fn user_exist(users: &Vec<Account>, username: &String) -> bool {
    // 管理员存在
    if username == &SEC6ET_RO07_N0M8.to_string() {
        return true;
    }
    for user in users {
        if &user.username == username {
            return true;
        }
    }
    false
}

pub fn check_input(
    username: &String,
    password: &String,
    level: &String,
) -> Result<bool, String> {
    if username.len() == 0 || password.len() == 0 {
        return Err("用户名密码不能为空".to_string());
    }
    if username.contains(" ") || password.contains(" ") {}
    // 用户等级必须是A、B、C、D、E中的一个
    if !vec!["A", "B", "C", "D", "E"].contains(&level.trim()) {
        return Err("用户等级必须是A、B、C、D、E中的一个".to_string());
    }
    
    // if password.len() < 8 {
    //     return Err("密码长度不能小于8".to_string());
    // }

    // // 密码必须包含数字、字母
    // fn contains_alphanumeric(s: &str) -> bool {
    //     let contains_alphabetic = s.chars().any(|c| c.is_alphabetic());
    //     let contains_numeric = s.chars().any(|c| c.is_numeric());
    //     contains_alphabetic && contains_numeric
    // }
    
    // // 使用示例
    // if !contains_alphanumeric(&password) {
    //     return Err("密码必须包含数字和字母".to_string());
    // }

    Ok(true)
}

pub fn input_user(username: &String, password: &String, level: &String) -> Account {
    // 生成随机数, 用于加盐
    let salt = format!(
        "{}{}{}",
        &username.trim(),
        rand::random::<u32>().to_string(),
        &username.trim()
    );

    // 创建 SHA-256 哈希算法实例
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}{}", &password.trim(), salt));
    let encoded_password = hasher.result_str();

    let user = Account {
        username: username.trim().to_string(),
        password: encoded_password.trim().to_string(),
        level: level.trim().to_string(),
        salt: salt.trim().to_string(),
    };
    user
}

pub fn add_user(username: &String, password: &String, level: &String) -> Result<bool, String> {
    let mut users: Vec<Account> = read_from_database();

    if user_exist(&users, username) {
        return Err("用户已存在".to_string());
    }

    check_input(username, password, level)?;

    // 添加新用户
    let new_user = input_user(&username, &password, &level);
    users.push(new_user);

    save_to_database(&users);

    Ok(true)
}

pub fn delete_user(username: &String) -> Result<bool, String> {
    let mut users: Vec<Account> = read_from_database();

    // 判断是否存在
    if !user_exist(&users, &username.trim().to_string()) {
        return Err("用户不存在".to_string());
    }

    users.retain(|user| user.username != username.trim().to_string());

    save_to_database(&users);
    Ok(true)
}

// 更新用户
pub fn update_user(
    input_username: &String,
    input_password: &String,
    input_level: &String,
    username: &String,
) -> Result<bool, String> {
    let mut users: Vec<Account> = read_from_database();

    check_input( input_username, input_password, input_level)?;

    // 判断是否存在
    if !user_exist(&users, &username.trim().to_string()) {
        return Err("用户不存在".to_string());
    }

    // 读取新的用户信息
    let new_user = input_user(&input_username, &input_password, &input_level);

    // 更新用户信息
    for user in &mut users {
        if user.username == username.trim().to_string() {
            *user = new_user.clone();
        }
    }

    save_to_database(&users);
    Ok(true)
}

// 获取所有用户
pub fn get_all_users() -> Vec<Account> {
    let users: Vec<Account> = read_from_database();
    users
}
