/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-22 23:06:16
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 19:07:39
 * @FilePath: \file-cryption\src\main.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use eframe::egui;
use file_encrypt::{
    app, cbc, create_file, file, hash, pem, rsa1,
    state_user::{self, get_priv_key, State},
    user_account::Account,
    users_db_connect, users_db_operate,
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 900.0)), // 设置窗口的宽度和高度
        resizable: false,                                     // 设置窗口是否可以调整大小
        hardware_acceleration: eframe::HardwareAcceleration::Required, // 设置是否使用硬件加速
        ..Default::default()                                  // 使用其他默认选项
    };
    // eframe::run_native(
    //     "puzzle",
    //     eframe::NativeOptions::default(),
    //     Box::new(|cc| Box::new(game::GameApp::new(|c|{

    //     }))),
    // )
    eframe::run_native(
        "WHO IS GOODPEANUTS",
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}

// fn main() {
//     // test();

//     // cmd_test();
// }

// fn cmd_test() {
//     let mut user: State;
//     println!("{}", WELCOME_MESSGAGE);
    // match user_state::login() {
    //     Some(u) => {
    //         user = State {
    //             account: u.clone(),
    //             priv_key: get_priv_key(&u.level),
    //         };
    //         println!("login success");
    //         println!("user: {}", u.username);
    //         println!("level: {}", u.level);
    //         for i in &user.priv_key {
    //             println!("{}", i);
    //         }
    //     }
    //     None => {
    //         println!("login failed");
    //         return;
    //     }
    // }
// }

// fn test() {
//     let mut test_mode = String::new();
//     println!("select run mode: ");
//     println!("0 test rsa");
//     println!("1 newfile for cbc speed test");
//     println!("2 cbc speedtest");
//     println!("3 rsa");
//     println!("4 add user");
//     println!("5 delete user");
//     println!("6 update user");
//     println!("7 show users");
//     println!("8 hash test");
//     println!("9 login");
//     println!("10 encrypt file");
//     println!("11 decrypt file");
//     println!("12 recover file");

//     std::io::stdin().read_line(&mut test_mode).unwrap();
//     match test_mode.trim() {
//         "0" => {
//             rsa1::test();
//         }
//         "1" => {
//             create_file::create_file();
//         }
//         "2" => {
//             cbc::speedtest();
//         }
//         "3" => {
//             rsa1::main();
//         }
//         "4" => {
//             users_db_operate::add_user();
//         }
//         "5" => {
//             users_db_operate::delete_user();
//         }
//         "6" => {
//             users_db_operate::update_user();
//         }
//         "7" => {
//             users_db_connect::show_users();
//         }
//         "8" => {
//             hash::test();
//         }
//         "9" => {
//             user_state::login();
//         }
//         "10" => {
//             file::encrypt_file("resources/10e");
//         }
//         "11" => {
//             file::decrypt_file("10e.txt", pem::PRIV_KEY_E);
//         }
//         "12" => {
//             file::recover_file("1a.txt");
//         }
//         _ => {
//             println!("no such mode");
//         }
//     }
// }

// const WELCOME_MESSGAGE: &str = r#"
//     Welcome to file encrypt system
//     0 test rsa
//     1 newfile for cbc speed test
//     2 cbc speedtest
//     3 rsa
//     4 add user
//     5 delete user
//     6 update user
//     7 show users
//     8 hash test
//     9 login
//     10 encrypt file
//     11 decrypt file
//     "#;
