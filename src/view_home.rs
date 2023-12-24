use crate::{app::App, state_user::State};
use eframe::egui::{self, Button, TextBuffer};
impl App {
    pub fn home(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.allocate_ui_with_layout(ui.available_size(), egui::Layout::top_down(egui::Align::Center), |ui|{
            ui.add_space(60.0);
            for (index, filename) in self.files.iter_mut().enumerate() {
                ui.add_space(10.0);
                let resp = ui.add_sized([200.0, 25.0], egui::Button::new(filename.to_string())).clicked();
                if resp {
                    // 如果按钮被点击，切换窗口状态
                    self.is_file_read_window_open[index] = true;
                }
            }
        });
    }
}