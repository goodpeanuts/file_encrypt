/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-24 00:36:27
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 02:24:35
 * @FilePath: \file-cryption\src\hash.rs
 * @Description: 测试hash函数
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */

use crypto::{digest::Digest, sha2::Sha256};

pub fn test() {

    // 创建 SHA-256 哈希算法实例
    let mut hasher1 = Sha256::new();
    let mut hasher2 = Sha256::new();

    // 输入数据到哈希算法
    hasher1.input_str(&format!("{}{}", "123456", "a"));

    // 计算哈希值
    let hash1_result = hasher1.result_str();

    println!("hash result: {}", hash1_result);

    // 输入数据
    println!("input password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();

    hasher2.input_str(&format!("{}{}", password.trim(), "a"));
    let hash2_result = hasher2.result_str();
    println!("hash result: {}", hash2_result);

    if hash1_result == hash2_result {
        println!("password correct");
    } else {
        println!("password incorrect");
    }
}
