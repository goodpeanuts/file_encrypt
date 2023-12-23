/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-22 23:06:16
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 01:22:21
 * @FilePath: /file_encrypt/src/create_file.rs
 * @Description: 
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