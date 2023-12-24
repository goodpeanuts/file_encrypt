/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-23 13:18:29
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 15:52:17
 * @FilePath: /file_encrypt/src/users_db_operate.rs
 * @Description: 用户数据库管理
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::{users_db_connect::{read_from_database, save_to_database},user_account::{Account, SEC6ET_RO07_N0M8}};
use crypto::{digest::Digest, sha2::Sha256};

pub fn user_exist(users: &Vec<Account>, username: &String) -> bool{
    // 管理员存在
    if username == &SEC6ET_RO07_N0M8.to_string()  {
        return true;
    }
    for user in users {
        if &user.username == username {
            return true;
        }
    }
    false
}

pub fn input_user (users: &Vec<Account>) -> Account {
    let mut username = String::new();
    let mut password = String::new();
    let mut level = String::new();

    println!("input username: ");
    std::io::stdin().read_line(&mut username).unwrap();
    
    // 判断是否存在重名
    while user_exist(&users, &username.trim().to_string()) {
        println!("username exist, input again: ");
        username.clear();
        std::io::stdin().read_line(&mut username).unwrap();
    }

    println!("input password: ");
    std::io::stdin().read_line(&mut password).unwrap();

    // 生成随机数, 用于加盐
    let salt = format!("{}{}{}", &username.trim(), rand::random::<u32>().to_string(), &username.trim());
    println!("salt: {}", salt);

    // 创建 SHA-256 哈希算法实例
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}{}", &password.trim(), salt));
    password = hasher.result_str();

    println!("input level: ");
    std::io::stdin().read_line(&mut level).unwrap();
    // 用户等级必须是A、B、C、D、E中的一个
    while !vec!["A", "B", "C", "D", "E"].contains(&level.trim()) {
        println!("level must be A, B, C, D or E, input again: ");
        level.clear();
        std::io::stdin().read_line(&mut level).unwrap();
    }

    let user = Account {
        username: username.trim().to_string(),
        password: password,
        level: level.trim().parse::<String>().unwrap(),
        salt: salt.trim().to_string(),
    };
    user
}


pub fn add_user() {
    let mut users: Vec<Account> = read_from_database();

    // 添加新用户
    let new_user = input_user(&users);
    users.push(new_user);

    save_to_database(&users);
}

pub fn delete_user() {
    let mut users: Vec<Account> = read_from_database();

    // 删除用户
    let mut username = String::new();
    println!("input username to delete: ");
    std::io::stdin().read_line(&mut username).unwrap();

    // 判断是否存在
    while !user_exist(&users, &username.trim().to_string()) {
        println!("user do not exist, input again: ");
        username.clear();
        std::io::stdin().read_line(&mut username).unwrap();
    }

    users.retain(|user| user.username != username.trim().to_string());

    save_to_database(&users);
}


// 更新用户
pub fn update_user() {
    let mut users: Vec<Account> = read_from_database();

    // 输入名称
    let mut username = String::new();
    println!("input username to update: ");
    std::io::stdin().read_line(&mut username).unwrap();

    // 判断是否存在
    while !user_exist(&users, &username.trim().to_string()) {
        println!("user do not exist, input again: ");
        username.clear();
        std::io::stdin().read_line(&mut username).unwrap();
    }

    // 读取新的用户信息
    let new_user = input_user(&users);

    // 更新用户信息
    for user in &mut users {
        if user.username == username.trim().to_string() {
            user.username = new_user.username.clone();
            user.password = new_user.password.clone();
            user.level = new_user.level.clone();
        }
    }

    save_to_database(&users);
}
