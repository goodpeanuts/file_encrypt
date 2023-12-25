use crate::file::decrypt_file;
/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-12-24 16:11:40
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 13:56:05
 * @FilePath: /file_encrypt/src/app.rs
 * @Description: 程序图形化界面主体
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::{config_font, file, state_nav, state_user, user_account, users_db_operate};
// use crate::{view_side_bar, view_login, state_nav::NavState, state_user::State};
use eframe::egui;
use std::path::Path;
use std::{fs, vec};
pub struct App {
    pub user: state_user::State,
    pub all_users: Vec<user_account::Account>,
    pub is_user_modify_window_open: Vec<bool>,
    pub input_username: String,
    pub input_password: String,
    pub input_level: String,
    pub is_login: bool,
    pub is_admin: bool,
    pub nav_state: state_nav::NavState,
    pub is_error: bool,
    pub error_message: String,
    pub adduser: bool,
    pub files: Vec<String>,
    pub is_file_read_window_open: Vec<bool>,
    pub file_content: Vec<String>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        config_font::custom_font(cc);
        App {
            user: state_user::State {
                account: user_account::Account {
                    username: "".to_string(),
                    password: "".to_string(),
                    salt: "".to_string(),
                    level: "".to_string(),
                },
                priv_key: vec![],
            },
            all_users: vec![],
            is_user_modify_window_open: vec![],
            input_username: "".to_string(),
            input_password: "".to_string(),
            input_level: "".to_string(),
            is_login: false,
            is_admin: false,
            nav_state: state_nav::NavState::Login,
            is_error: false,
            error_message: "".to_string(),
            adduser: false,
            files: vec![],
            is_file_read_window_open: vec![],
            file_content: vec![],
        }
    }

    pub fn logout(&mut self) {
        self.user.account = user_account::Account {
            username: "".to_string(),
            password: "".to_string(),
            salt: "".to_string(),
            level: "".to_string(),
        };
        self.input_password = "".to_string();
        self.input_username = "".to_string();
        self.input_level = "".to_string();
        self.user.priv_key = vec![];
        self.all_users = vec![];
        self.is_user_modify_window_open = vec![];
        self.is_login = false;
        self.is_admin = false;
        self.nav_state = state_nav::NavState::Login;
        self.is_error = false;
        self.error_message = "".to_string();
        self.adduser = false;
        self.files = vec![];
        self.is_file_read_window_open = vec![];
        self.file_content = vec![];
    }

    pub fn read_filenames_in_dir(&mut self) -> std::io::Result<()> {
        let dir = Path::new("ciphertext");
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            self.files.push(filename_str.to_string());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // 修改用户
    fn modify_user(&mut self, ctx: &egui::Context) {
        for (index, is_open) in self.is_user_modify_window_open.iter_mut().enumerate() {
            egui::Window::new(self.all_users[index].username.clone())
                .title_bar(true)
                .open(is_open)
                .default_open(true)
                .constrain(true)
                .collapsible(false)
                .movable(true)
                .show(ctx, |ui| {});
        }
    }

    // 添加用户
    fn add_user(&mut self, ctx: &egui::Context) {
        egui::Window::new("添加用户")
            .title_bar(true)
            .default_pos([500.0, 200.0])
            .fixed_size([600.0, 900.0])
            .open(&mut self.adduser)
            .default_size([600.0, 900.0])
            .default_open(true)
            .constrain(true)
            .collapsible(false)
            .movable(true)
            .show(ctx, |ui: &mut egui::Ui| {
                ui.add_space(20.0);
                ui.add_sized(
                    egui::vec2(200.0, 24.0),
                    egui::TextEdit::singleline(&mut self.input_username).hint_text("账号"),
                );
                ui.add_space(10.0);
                ui.add_sized(
                    egui::vec2(200.0, 24.0),
                    egui::TextEdit::singleline(&mut self.input_password)
                        .hint_text("密码")
                        .password(true),
                );
                ui.add_space(10.0);
                ui.add_sized(
                    egui::vec2(200.0, 24.0),
                    egui::TextEdit::singleline(&mut self.input_level).hint_text("用户等级"),
                );
                if self.is_error {
                    ui.label(egui::RichText::new(&self.error_message).color(egui::Color32::RED));
                }
                ui.add_space(10.0);
                let resp_add = ui
                    .add_sized(
                        [200.0, 32.0],
                        egui::Button::new(egui::RichText::new("添加").size(16.0)),
                    )
                    .clicked();
                if resp_add {
                    match users_db_operate::add_user(
                        &self.input_username,
                        &self.input_password,
                        &self.input_level,
                    ) {
                        Ok(_) => {
                            self.is_error = false;
                            self.input_username.clear();
                            self.input_password.clear();
                            self.input_level.clear();
                            self.all_users = users_db_operate::get_all_users();
                            self.is_user_modify_window_open.push(false);
                        }
                        Err(e) => {
                            self.is_error = true;
                            self.error_message = e.clone();
                            ui.label(egui::RichText::new(e).color(egui::Color32::RED));
                        }
                    }
                }
            });
    }
    fn show_file(&mut self, ctx: &egui::Context) {
        for (index, is_open) in self.is_file_read_window_open.iter_mut().enumerate() {
            egui::Window::new(self.files[index].clone())
                .title_bar(true)
                .open(is_open)
                .default_open(true)
                .constrain(true)
                .collapsible(false)
                .movable(true)
                .show(ctx, |ui| {
                    if self.file_content[index] == "".to_string() {
                        if self.user.account.level == "A" {
                            match file::recover_file(&self.files[index]) {
                                Ok(c) => self.file_content[index] = c,
                                Err(e) => {
                                    self.file_content[index] = e.clone().to_string();
                                }
                            }
                        } else {
                            for k in self.user.priv_key.iter() {
                                match decrypt_file(&self.files[index], k) {
                                    Ok(c) => self.file_content[index] = c,
                                    Err(e) => {
                                        self.file_content[index] = e.clone().to_string();
                                    }
                                }
                                if self.file_content[index] != "无权限访问".to_string() {
                                    break;
                                }
                            }
                        }
                    };

                    if "无权限访问".to_string() != self.file_content[index] {
                        ui.add_space(20.0);
                        ui.add_sized(
                            egui::vec2(320.0, 24.0),
                            egui::TextEdit::multiline(&mut self.file_content[index])
                                .interactive(false),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(&self.file_content[index])
                                .color(egui::Color32::RED)
                                .size(18.0),
                        );
                    }
                });
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.nav_state {
            state_nav::NavState::Login => {
                self.login(ctx, ui);
            }
            state_nav::NavState::Home => {
                self.home(ctx, ui);
                self.side_bar(ctx, ui);
                // if !self.game_state.end && self.game_state.bot {
                //     self.recover();
                // }
                self.modify_user(ctx);
                self.add_user(ctx);
                self.show_file(ctx);
            }
        });
    }
}
