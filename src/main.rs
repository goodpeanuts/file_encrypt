/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-22 23:06:16
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 03:17:53
 * @FilePath: /file_encrypt/src/main.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use file_encrypt::{create_file, cbc, rsa1, users_db_operate, users_db_connect, hash, file};

fn main() {
    let mut test_mode = String::new();
    println!("select run mode: ");
    println!("0 test rsa");
    println!("1 newfile for cbc speed test");
    println!("2 cbc speedtest");
    println!("3 rsa");
    println!("4 add user");
    println!("5 delete user");
    println!("6 update user");
    println!("7 show users");
    println!("8 hash test");
    println!("9 login");
    println!("10 encrypt file");
    println!("11 decrypt file");
    
    std::io::stdin().read_line(&mut test_mode).unwrap();
    match test_mode.trim() {
        "0" => {
            rsa1::test();
        },
        "1" => {
            create_file::create_file();
        },
        "2"  => {
            cbc::speedtest();
        },
        "3" => {
            rsa1::main();
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
        "8" => {
            hash::test();
        },
        "9" => {
            users_db_operate::login();
        }
        "10" => {
            file::encrypt_file("file.txt");
        }
        "11" => {
            file::decrypt_file("good.txt");}
        _ => {
            println!("no such mode");
        } 
    }
}
