/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-22 23:06:16
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 02:22:30
 * @FilePath: \file-cryption\src\create_file.rs
 * @Description: 创建一个用于测试cbc加密解密的文件
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn create_file() {
    let file = File::create("file.txt").expect("file not found");
    let mut writer = BufWriter::new(file);
    
    print!("Please input file size: ");
    std::io::stdout().flush().unwrap();

    // 输入一个数字
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("read failed");
    let input: usize = input.trim().parse().expect("parse failed");

    for i in 0..input {
        writer.write_all(format!("{}       good for test\n", i + 1).as_bytes()).expect("write failed");
    }
}