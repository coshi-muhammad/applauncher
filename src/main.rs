use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use eframe::egui;
use egui::*;

struct AppState {
    name: String,
    highlighted: u32,
    items: Vec<Item>,
}

struct Item {
    name: String,
    launch_cmd: String,
}

impl Default for AppState {
    fn default() -> Self {
        let item_list = vec![
            Item {
                name: "zen".to_owned(),
                launch_cmd: "/usr/bin/zen-browser".to_owned(),
            },
            Item {
                name: "vesktop".to_owned(),
                launch_cmd: "/usr/bin/vesktop".to_owned(),
            },
            Item {
                name: "dolphin".to_owned(),
                launch_cmd: "/usr/bin/dolphin".to_owned(),
            },
            Item {
                name: "obsidian".to_owned(),
                launch_cmd: "/usr/bin/obsidian".to_owned(),
            },
            Item {
                name: "telegram".to_owned(),
                launch_cmd: "/usr/bin/Telegram".to_owned(),
            },
            Item {
                name: "text editor".to_owned(),
                launch_cmd: "/usr/bin/flatpak run org.gnome.TextEditor".to_owned(),
            },
        ];

        AppState {
            name: "launch app".to_owned(),
            highlighted: 0,
            items: item_list,
        }
    }
}

fn show_item(ui: &mut egui::Ui, item: &Item, is_highlited: bool) -> bool {
    let mut clicked = false;
    let frame: egui::Frame = if is_highlited {
        let background_color = Color32::from_hex("#1e1e2e").unwrap_or(Color32::RED);
        let border_color = Color32::from_hex("#8da9cb").unwrap_or(Color32::YELLOW);
        let border = egui::Stroke::from((3.0, border_color));
        egui::Frame::default().fill(background_color).stroke(border)
    } else {
        let background_color = Color32::from_hex("#181825").unwrap_or(Color32::RED);
        let border_color = Color32::from_hex("#585858").unwrap_or(Color32::YELLOW);
        let border = egui::Stroke::from((3.0, border_color));
        egui::Frame::default().fill(background_color).stroke(border)
    };

    let layout = egui::Layout::top_down(egui::Align::Center).with_main_wrap(false);
    ui.allocate_ui_with_layout(egui::vec2(ui.available_width(), 0.0), layout, |ui| {
        let frame_result = frame.show(ui, |ui| ui.label(item.name.clone()));
        let interaction = ui.interact(
            frame_result.response.rect,
            frame_result.response.id.with("click_sense"),
            egui::Sense::click(),
        );
        if interaction.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if interaction.clicked() {
            clicked = true;
        }
    });
    clicked // rust wants to be haskell so bad they let you put a variable at the end without a
    // semicolon to return it
}

fn launch_app(lauch_command: &str) {
    if lauch_command.starts_with("/usr/bin/flatpak") {
        let paramaters: Vec<&str> = lauch_command.split(" ").collect();
        Command::new(paramaters[0])
            .arg(paramaters[1])
            .arg(paramaters[2])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0)
            .spawn()
            .expect("Failed to launch");
    } else {
        Command::new(format!("{}", lauch_command))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0)
            .spawn()
            .expect("Failed to launch");
    }
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let mut should_close = false;
        //main loop
        // draw ui and handle widget specific events
        egui::CentralPanel::default_margins().show_inside(ui, |ui| {
            let this: &AppState = self;
            for (i, item) in (1..).zip(self.items.iter()) {
                if show_item(ui, item, i == self.highlighted) {
                    println!("{} should launch", item.name);
                    launch_app(&item.launch_cmd);
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
                println!("{} should launch", self.items[lauched_item_idx].name);
                //HACK: for now its special cased untill i find a better way of dealing with it
                launch_app(&self.items[lauched_item_idx].launch_cmd);
                should_close = true;
            }
            if input.key_pressed(egui::Key::Tab) || input.key_pressed(egui::Key::ArrowDown) {
                self.highlighted += 1;
                if self.highlighted > self.items.len() as u32 {
                    self.highlighted -= (self.items.len() + 1) as u32;
                }
            }
            if input.key_pressed(egui::Key::ArrowUp) {
                if self.highlighted == 0 {
                    self.highlighted = self.items.len() as u32;
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

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "applauncher",
        options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
}
