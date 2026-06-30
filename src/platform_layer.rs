#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    //TODO: Change launch_cmd to a command type that can be constructed by a string
    launch_cmd: LaunchCmd,
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
            launch_cmd: LaunchCmd::default(),
            kind: None,
            icon: None,
            description: None,
            terminal_app: None,
            keywords: None,
        }
    }
}
#[derive(Clone, Debug, Default)]
struct LaunchCmd {
    arguments: Vec<String>,
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
    // use std::os::unix::process::CommandExt;
    use std::path::Path;
    use std::process::{Command, Stdio};
    // enum DesktopEntryErr {
    //     IO(io::Error),
    //     InvalidFormat(String),
    //     MissingFiled(&'static str),
    // }
    impl LaunchCmd {
        //TODO: change the return type to be a result type that errors in error cases for the
        //parcing
        //HACK: Definatly need to optimize this bitch
        fn new(source: String, item: &Item) -> Self {
            let mut result = LaunchCmd::default();
            //remove unessasery characters
            let mut descaped_string = String::default();
            let mut quoted = false;
            let mut escaped = false;
            //TODO: change this to use more robust parcing with actuall types
            for char in source.chars() {
                match char {
                    '"' => {
                        if quoted && escaped {
                            descaped_string.push(char);
                            escaped = false;
                        } else if quoted && !escaped {
                            quoted = false;
                        } else if !quoted {
                            quoted = true;
                        } else {
                            //should error
                        }
                    }
                    '\\' => {
                        if quoted && escaped {
                            descaped_string.push(char);
                            escaped = false;
                        } else if quoted && !escaped {
                            escaped = true;
                        } else {
                            //should error
                        }
                    }
                    '`' => {
                        if quoted && escaped {
                            descaped_string.push(char);
                            escaped = false;
                        } else {
                            //should error
                        }
                    }
                    '$' => {
                        if quoted && escaped {
                            descaped_string.push(char);
                            escaped = false;
                        } else {
                            //should error
                        }
                    }
                    ' ' => {
                        if quoted {
                            descaped_string.push_str("~~|");
                        } else {
                            descaped_string.push(char);
                        }
                    }
                    '\t' | '\n' | '\'' | '>' | '<' | '~' | '|' | '&' | ';' | '*' | '#' | '?'
                    | '(' | ')' => {
                        if quoted {
                            descaped_string.push(char);
                        } else {
                            //should error
                        }
                    }
                    _ => {
                        descaped_string.push(char);
                    }
                }
            }
            if quoted {
                //should error
            }
            // spliting the arguments by white space
            result.arguments = descaped_string
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            // replacing the marker with an actuall space
            result.arguments = result
                .arguments
                .iter()
                .map(|s| s.replace("~~|", " "))
                .collect();
            // handeling filed codes
            // use filter_map when you deside to handle the the other cases
            result.arguments.retain(|s| {
                !matches!(
                    s.as_str(),
                    "%i" | "%c"
                        | "%k"
                        | "%f"
                        | "%F"
                        | "%u"
                        | "%U"
                        | "%d"
                        | "%D"
                        | "%n"
                        | "%N"
                        | "%v"
                        | "%m"
                )
            });
            //TODO: see if you can avoid this clone
            if !result.arguments.is_empty() {
                result.arguments[0] = get_path(result.arguments[0].clone());
            } else {
                //should error
            }
            result
        }
    }

    fn get_path(source: String) -> String {
        if source.starts_with("/") {
            return source;
        }
        let path_dirs = env::var("PATH").unwrap();
        for path in path_dirs.split(":") {
            let program_path = format!("{path}/{source}");
            if Path::new(&program_path).exists() {
                return program_path;
            }
        }
        String::new()
    }
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
        if let Some((program, args)) = app.launch_cmd.arguments.split_first() {
            Command::new(program)
                .args(args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
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
        let mut exec_string = String::new();
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
                    exec_string = line.split("=").nth(1).unwrap().to_string();
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

        item.launch_cmd = LaunchCmd::new(exec_string, &item);
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
