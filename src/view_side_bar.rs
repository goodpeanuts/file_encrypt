/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-12-24 16:25:47
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 02:27:11
 * @FilePath: \file-cryption\src\view_side_bar.rs
 * @Description:    侧边栏
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::app::App;
use eframe::egui;

impl App {
    pub fn side_bar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let game_side_rect =
            egui::Rect::from_min_max(egui::pos2(900.0, 20.0), egui::pos2(1200.0, 900.0));
        ui.allocate_ui_at_rect(game_side_rect, |ui| {
            ui.allocate_ui_with_layout(
                ui.available_size(),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.add_space(100.0);

                    // 是否是管理员
                    if !self.is_admin {
                        ui.add_sized(
                            [120.0, 40.0],
                            egui::SelectableLabel::new(
                                !self.is_admin,
                                egui::RichText::new(format!(
                                    "{}{}",
                                    "👤  ", self.user.account.level
                                ))
                                .size(22.0),
                            ),
                        );
                    } else {
                        ui.add_sized(
                            [120.0, 40.0],
                            egui::SelectableLabel::new(
                                self.is_admin,
                                egui::RichText::new(format!(
                                    "{}{}",
                                    "🎉  ", self.user.account.level
                                ))
                                .size(22.0),
                            ),
                        );

                        ui.add_space(20.0);
                        for (index, user) in self.all_users.iter_mut().enumerate() {
                            ui.add_space(10.0);
                            let resp = ui
                                .add_sized(
                                    [200.0, 25.0],
                                    egui::Button::new(format!("{}{}", "👤  ", user.username)),
                                )
                                .clicked();
                            if resp {
                                // 如果按钮被点击，切换窗口状态
                                self.is_user_modify_window_open[index] = true;
                            }
                        }

                        ui.add_space(10.0);
                        let resp = ui
                            .add_sized([200.0, 25.0], egui::Button::new("添加新用户"))
                            .clicked();
                        if resp {
                            // 如果按钮被点击，切换窗口状态
                            self.adduser = true;
                        }
                    }

                    ui.add_space(50.0);

                    ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_BLUE;

                    // 这里重开一个ui，不然按钮的长度会因为justified被强制拉长至和layout一样长
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);

                        ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::RED;
                        let return_resp = ui.add_sized(
                            [120.0, 40.0],
                            egui::Button::new(egui::RichText::new("退出").size(17.0)),
                        );

                        if return_resp.clicked() {
                            self.logout()
                        }

                        // if self.game_state.end && !self.game_state.win {
                        //     //ui.is_visible();
                        //     ui.add_sized(
                        //         [80.0, 19.0],
                        //         egui::Label::new("You shall be better next time"),
                        //     );
                        // }
                    });
                    // ui.add_space(60.0);
                },
            )
        });
    }
}
