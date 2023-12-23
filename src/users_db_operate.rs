/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-23 13:18:29
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 01:16:57
 * @FilePath: /file_encrypt/src/users_db_operate.rs
 * @Description: 用户管理
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::{users_db_connect::{open_account_databases, close_account_databases},user_type::Account};
use crypto::{digest::Digest, sha2::Sha256};

fn user_exist(users: &Vec<Account>, username: &String) -> bool{
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
    let mut salt = String::new();

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
    salt = format!("{}{}{}", &username.trim(), rand::random::<u32>().to_string(), &username.trim());
    println!("salt: {}", salt);

    // 创建 SHA-256 哈希算法实例
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}{}", &password.trim(), salt));
    password = hasher.result_str();

    println!("input level: ");
    std::io::stdin().read_line(&mut level).unwrap();

    let user = Account {
        username: username.trim().to_string(),
        password: password,
        level: level.trim().parse::<String>().unwrap(),
        salt: salt.trim().to_string(),
    };
    user
}


pub fn add_user() {
    let mut users: Vec<Account> = open_account_databases();

    // 添加新用户
    let new_user = input_user(&users);
    users.push(new_user);

    close_account_databases(&users);
}

pub fn delete_user() {
    let mut users: Vec<Account> = open_account_databases();

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

    close_account_databases(&users);
}


// 更新用户
pub fn update_user() {
    let mut users: Vec<Account> = open_account_databases();

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

    close_account_databases(&users);
}

pub fn login() -> bool {
    let users: Vec<Account> = open_account_databases();

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

    // 获取对应用户名的密码
    let mut password = String::new();
    let mut salt = String::new();
    for user in &users {
        if user.username == username.trim().to_string() {
            password = user.password.clone();
            salt = user.salt.clone();
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
        return true;
    } else {
        println!("login failed");
    }

    false
}
