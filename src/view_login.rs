/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-12-24 16:15:10
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 23:03:51
 * @FilePath: \file-cryption\src\view_login.rs
 * @Description: 登录界面
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::{app::App, state_user::{State, self}, state_nav, users_db_operate};
use eframe::egui::{self, Button};

impl App {
    pub fn login(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.allocate_at_least(egui::vec2(1200.0, 400.0), egui::Sense::hover());
        ui.allocate_ui_with_layout(
            egui::vec2(1200.0, 400.0),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.vertical_centered(|ui| {
                    // ui.label(egui::RichText::new("Jigsaw Puzzle").size(32.0).color(egui::Color32::LIGHT_GRAY));
                    ui.label(egui::RichText::new("✨ Jigsaw Puzzle").size(32.0));
                    ui.add_space(50.0);
                    ui.add_sized(egui::vec2(200.0, 24.0), egui::TextEdit::singleline(&mut self.input_username).hint_text("账号"));
                    ui.add_space(10.0);
                    ui.add_sized(egui::vec2(200.0, 24.0), egui::TextEdit::singleline(&mut self.input_password).hint_text("密码").password(true));
                    ui.add_space(10.0);
                    let mut resp_login = ui
                    .add_sized([130.0, 35.0], Button::new(egui::RichText::new("登录").size(16.0)))
                    .clicked();

                    if resp_login {
                        println!("{}", self.input_username);
                        println!("{}", self.input_password);
                        match state_user::login(&self.input_username, &self.input_password) {
                            Some(u) => {
                                self.user = State {
                                    account: u.clone(),
                                    priv_key: state_user::get_priv_key(&u.level),
                                };
                                println!("login success");
                                println!("user: {}", u.username);
                                println!("level: {}", u.level);
                                for i in &self.user.priv_key {
                                    println!("{}", i);
                                }
                                self.is_login = true;
                                if u.level == "A" {
                                    self.is_admin = true;
                                    self.all_users = users_db_operate::get_all_users();
                                    self.is_user_modify_window_open = vec![false; self.all_users.len()];
                                }
                                self.read_filenames_in_dir();
                                self.nav_state = state_nav::NavState::Home;
                                self.input_password.clear();
                                self.input_username.clear();
                            }
                            None => {
                                self.is_error = true;
                                println!("login failed");
                                self.input_password.clear();
                            }
                        }

                    }

                    ui.add_space(10.0);
                    if self.is_error {
                        ui.label(egui::RichText::new("账号或密码错误").color(egui::Color32::RED));
                    }
                    
                });
            },
        );
    }
}
