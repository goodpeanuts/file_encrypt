/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-12-24 17:42:15
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 02:22:08
 * @FilePath: \file-cryption\src\config_font.rs
 * @Description: 字体配置文件
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use eframe::egui;

pub fn custom_font(cc: &eframe::CreationContext<'_>){
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "OPPOSans-L".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/fonts/YaiHe.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "OPPOSans-L".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("OPPOSans-L".to_owned());

    // Tell egui to use these fonts:
    cc.egui_ctx.set_fonts(fonts);
}