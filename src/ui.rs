use egui::*;

use super::platform_layer::{Item, launch_app, load_items, search_item};

pub struct AppState {
    name: String,
    highlighted: u32,
    search_pattern: String,
    items: Vec<Item>,
    searched_list: Vec<Item>,
}

impl Default for AppState {
    fn default() -> Self {
        let item_list = load_items();

        AppState {
            name: "launch app".to_owned(),
            highlighted: 0,
            search_pattern: "".to_owned(),
            items: item_list.clone(),
            searched_list: item_list.clone(),
        }
    }
}

fn show_item(ui: &mut egui::Ui, item: &Item, highlited: bool) -> (bool, bool) {
    let mut clicked = false;
    let mut hovered = false;
    let background_color = if highlited {
        Color32::from_hex("#1e1e2e").unwrap_or(Color32::RED)
    } else {
        Color32::from_hex("#181825").unwrap_or(Color32::RED)
    };
    let border_color = if highlited {
        Color32::from_hex("#8da9cb").unwrap_or(Color32::YELLOW)
    } else {
        Color32::from_hex("#585858").unwrap_or(Color32::YELLOW)
    };
    let border = egui::Stroke::from((3.0, border_color));
    let frame: egui::Frame = egui::Frame::default().fill(background_color).stroke(border);

    let layout = egui::Layout::top_down(egui::Align::Center).with_main_wrap(false);
    ui.allocate_ui_with_layout(egui::vec2(ui.available_width(), 0.0), layout, |ui| {
        let mut prepared_frame = frame.begin(ui);
        {
            prepared_frame.content_ui.label(item.name.clone());
        }
        let response = prepared_frame.allocate_space(ui);
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            hovered = true;
        }
        if response.clicked() {
            clicked = true;
        }
        prepared_frame.paint(ui);
    });
    (clicked, hovered) // rust wants to be haskell so bad they let you put a variable at the end without a
    // semicolon to return it
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let mut should_close = false;
        //main loop
        // draw ui and handle widget specific events

        egui::CentralPanel::default_margins().show_inside(ui, |ui| {
            let search_bar_response = ui.add(
                egui::TextEdit::singleline(&mut self.search_pattern).hint_text("app name + enter"),
            );

            search_bar_response.request_focus();
            if search_bar_response.changed() {
                self.searched_list = search_item(self.items.clone(), self.search_pattern.clone());
            }

            for (i, item) in (1..).zip(self.searched_list.iter()) {
                let (clicked, hovered) = show_item(ui, item, i == self.highlighted);
                if hovered {
                    self.highlighted = i;
                }
                if clicked {
                    println!("{} should launch", item.name);
                    launch_app(item);
                    self.highlighted = i;
                    should_close = true;
                }
            }
        });

        //handle global key events and shortcuts
        ui.input(|input| {
            if input.key_pressed(egui::Key::Enter) {
                let lauched_item_idx = if self.highlighted == 0 {
                    0
                } else {
                    self.highlighted - 1
                } as usize;
                println!(
                    "{} should launch",
                    self.searched_list[lauched_item_idx].name
                );
                launch_app(&self.searched_list[lauched_item_idx]);
                should_close = true;
            }
            if input.key_pressed(egui::Key::Tab) || input.key_pressed(egui::Key::ArrowDown) {
                self.highlighted += 1;
                if self.highlighted > self.searched_list.len() as u32 {
                    self.highlighted -= (self.searched_list.len() + 1) as u32;
                }
            }
            if input.key_pressed(egui::Key::ArrowUp) {
                if self.highlighted == 0 {
                    self.highlighted = self.searched_list.len() as u32;
                } else {
                    self.highlighted -= 1;
                }
            }
            if input.key_pressed(egui::Key::Escape) {
                should_close = true;
            }
        });
        if should_close {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}
