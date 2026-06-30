#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    //TODO: Change launch_cmd to a command type that can be constructed by a string
    launch_cmd: String,
    kind: Option<String>,
    pub icon: Option<String>,
    description: Option<String>,
    pub terminal_app: Option<bool>,
    keywords: Option<String>,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            name: "".to_string(),
            launch_cmd: "".to_string(),
            kind: None,
            icon: None,
            description: None,
            terminal_app: None,
            keywords: None,
        }
    }
}
#[cfg(target_os = "linux")]
mod linux {

    use super::*;
    use freedesktop_icon::get_icon;
    use std::env;
    use std::fs::File;
    use std::fs::read_dir;
    use std::io;
    use std::io::prelude::*;
    use std::os::unix::process::CommandExt;
    use std::path::Path;
    use std::process::{Command, Stdio};
    // enum DesktopEntryErr {
    //     IO(io::Error),
    //     InvalidFormat(String),
    //     MissingFiled(&'static str),
    // }

    fn load_desktop_file_list() -> io::Result<Vec<String>> {
        //FIX: remove douplicate entries using a praiority model
        let mut files: Vec<String> = vec![];
        for entry in read_dir("/usr/share/applications/")? {
            let path_string = entry?.path().to_str().unwrap().to_owned();
            if path_string.ends_with(".desktop") {
                files.push(path_string);
            }
        }
        for entry in read_dir("/usr/local/share/applications/")? {
            let path_string = entry?.path().to_str().unwrap().to_owned();
            if path_string.ends_with(".desktop") {
                files.push(path_string);
            }
        }
        for entry in read_dir(format!(
            "{}/.local/share/applications/",
            env::var("HOME").unwrap()
        ))? {
            let path_string = entry?.path().to_str().unwrap().to_owned();
            if path_string.ends_with(".desktop") {
                files.push(path_string);
            }
        }
        Ok(files)
    }

    pub fn launch_app(app: &Item) {
        if app.launch_cmd.starts_with("/usr/bin/flatpak") {
            //HACK: for now its special cased untill i find a better way of dealing with it
            let paramaters: Vec<&str> = app.launch_cmd.split(" ").collect();

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
            Command::new(format!("{}", app.launch_cmd))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .process_group(0)
                .spawn()
                .expect("Failed to launch");
        }
    }

    fn parce_desktop_file(path: &String) -> Result<Item, io::Error> {
        //TODO: make a builder type for item
        let mut item: Item = Item::default();
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        //TODO: make your own ini parcer and use it for this at some point
        if !contents.starts_with("[Desktop Entry]") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{} is not a valid desktop file missing [Desktop Entry]",
                    path
                ),
            ));
        }
        for line in contents.lines() {
            let line = line.trim();
            match line {
                "" => continue,
                line if line.starts_with("#") || line.starts_with(";") => continue,
                line if line.starts_with("[") => {
                    if line != "[Desktop Entry]" {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "{} is not a supported desktop file It has an extra group {}",
                                path, line
                            ),
                        ));
                    }
                }
                line if line.starts_with("Exec=") => {
                    item.launch_cmd = line.split("=").nth(1).unwrap().to_string();
                }
                line if line.starts_with("Name=") => {
                    //TODO: support localized names at some point
                    item.name = line.split("=").nth(1).unwrap().to_string();
                }
                line if line.starts_with("Type=") => {
                    if line.split("=").nth(1) != Some("Application") {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "{} is not a supported desktop file we only work with applications",
                                path
                            ),
                        ));
                    }
                }
                line if line.starts_with("GenericName=") => {
                    item.kind = Some(line.split("=").nth(1).unwrap().to_string());
                }
                line if line.starts_with("Icon=") => {
                    //TODO: use the icon crate to handel this
                    let icon = line.split("=").nth(1).unwrap().to_string();
                    if icon.starts_with("/") {
                        if Path::new(&icon).exists() {
                            item.icon = Some(icon);
                        }
                    } else {
                        let icon_path = Some(
                            get_icon(&icon)
                                .unwrap_or_default()
                                .to_str()
                                .unwrap()
                                .to_owned(),
                        );
                        if icon_path != Some(String::default()) {
                            item.icon = icon_path;
                        }
                    }
                }
                line if line.starts_with("Comment=") => {
                    item.description = Some(line.split("=").nth(1).unwrap().to_string());
                }
                line if line.starts_with("Keywords=") => {
                    item.keywords = Some(line.split("=").nth(1).unwrap().to_string());
                }
                line if line.starts_with("Terminal=") => {
                    item.terminal_app =
                        Some(line.split("=").nth(1).unwrap().to_string().parse().unwrap());
                }

                &_ => continue,
            }
        }
        Ok(item)
    }

    pub fn load_items() -> Vec<Item> {
        let mut items = vec![];
        let files = load_desktop_file_list().unwrap_or(vec![]);
        for file in files {
            let mut should_continue = false;
            let item = parce_desktop_file(&file).unwrap_or_else(|_| {
                should_continue = true;
                Item::default()
            });
            if !should_continue {
                items.push(item);
            }
        }
        items
    }
}

#[cfg(target_os = "windows")]
mod windows {

    #[cfg(target_os = "windows")]
    pub fn load_items() -> Vec<Item> {
        vec![]
    }
    #[cfg(target_os = "windows")]
    pub fn launch_app(app: &Item) {}
}

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
pub use windows::*;

pub fn search_item(app_list: Vec<Item>, search_pattern: String) -> Vec<Item> {
    app_list
        .iter()
        .filter(|item| item.name.starts_with(&search_pattern))
        .cloned()
        .collect()
}
