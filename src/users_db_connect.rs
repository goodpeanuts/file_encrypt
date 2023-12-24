/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-23 18:36:00
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 22:14:18
 * @FilePath: \file-cryption\src\users_db_connect.rs
 * @Description: 用于打开以及加密和解密的用户信息文件
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use std::io::{Read, Write};
use crypto::aes;

use crate::{user_account::Account, users_db_operate, cbc};

// 指定固定的密钥和IV，实际应用中需要使用安全的随机生成的密钥和IV
const KEY256: &[u8] = b"0123456789abcdef0123456789abcdef"; // 用于256的测试密钥
const IV: &[u8] = b"0123456789abcdef";

// gui 中需要重写
// 当用户信息文件不存在时，创建用户信息文件 
// pub fn create_database(count: i32) -> Vec<Account> {
//     let mut users: Vec<Account> = Vec::new();
//     let mut count = String::new();

//     println!("input users number: ");
//     std::io::stdin().read_line(&mut count).unwrap();

//     for _i in 0..count.trim().parse::<i32>().unwrap() {
//         let user = users_db_operate::input_user(&users);
//         users.push(user);
//     }
//     users
// }

pub fn read_from_database() -> Vec<Account> {
    // 打开文件并读取内容
    let file = std::fs::File::open("users.json");
    match file {
        Ok(mut file) => {
            let mut ciphertext = Vec::new();
            file.read_to_end(&mut ciphertext).unwrap();
            let plaintext = cbc::aes_cbc_decrypt(&mut ciphertext, aes::KeySize::KeySize256, &KEY256, &IV);
        
            // 解析为用户数组
            let users: Vec<Account> = serde_json::from_slice(&plaintext).unwrap();
            println!();    // for test
            println!("== open users database success ==");    // for test   
            println!();    // for test
            print_users(&users);    // for test
            println!();    // for test
            println!("== open end ==");    // for test
            println!();    // for test
            users
        }
        Err(_) => {
            println!("!! open users database failed !!");    // for test
            println!();    // for test
            println!("== create new users database ==");    // for test
            // let users = create_database();   // 暂时返回空数组
            let users = vec![];
            print_users(&users);    // for test
            users
        }
    }
}


pub fn save_to_database(users: &Vec<Account>) {
    // 将用户数组转换为json字符串
    let plaintext = serde_json::to_string_pretty(&users).unwrap();
    let plaintext = plaintext.as_bytes();
    let mut ciphertext = cbc::aes_cbc_encrypt(&plaintext, aes::KeySize::KeySize256, &KEY256, &IV);

    let mut file = std::fs::File::create("users.json").unwrap();
    file.write_all(&mut ciphertext).unwrap();
    
    println!();    // for test
    println!("== close users database success ==");    // for test   
    println!();    // for test
    print_users(&users);    // for test
    println!();    // for test
    println!("== close end ==");    // for test
    println!();    // for test

}

// for test 查看明文的用户信息
pub fn show_users() {
    let users = read_from_database();
    // 保存到新文件
    let mut file = std::fs::File::create("users_decrypt.json").unwrap();
    let json = serde_json::to_string_pretty(&users).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

/**
 * @description: 打印用户信息到终端
 * @for debug
 */
pub fn print_users(users: &Vec<Account>) {
    for user in users {
        println!("username: {}", user.username);
        println!("password: {}", user.password);
        println!("level: {}", user.level);
    }
}
