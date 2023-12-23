/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-22 23:06:16
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 00:02:19
 * @FilePath: /file_encrypt/src/main.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use file_encrypt::{newfile, cbc, rsa1, users_db_operate, users_db_connect};

fn main() {
    let mut test_mode = String::new();
    println!("select run mode: ");
    println!("1 newfile for cbc speed test");
    println!("2 cbc speedtest");
    println!("3 rsa");
    println!("4 add user");
    println!("5 delete user");
    println!("6 update user");
    println!("7 show users");
    
    std::io::stdin().read_line(&mut test_mode).unwrap();
    match test_mode.trim() {
        "1" => {
            newfile::output_file();
        },
        "2"  => {
            cbc::speedtest();
        },
        "3" => {
            rsa1::rsa_crypt();
        },
        "4" => {
            users_db_operate::add_user();
        },
        "5" => {
            users_db_operate::delete_user();
        },
        "6" => {
            users_db_operate::update_user();
        },
        "7" => {
            users_db_connect::show_users();
        },
        _ => {
            println!("no such mode");
        } 
    }
}
