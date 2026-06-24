use std::env;
use std::fs::*;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    launch_cmd: String,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            name: "".to_string(),
            launch_cmd: "".to_string(),
        }
    }
}

#[cfg(target_os = "linux")]
fn load_desktop_file_list() -> io::Result<Vec<String>> {
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

#[cfg(target_os = "linux")]
fn parce_desktop_file(path: String) -> Result<Item, io::Error> {
    let item: Item = Item::default();
    //TODO: read the file and extract what we care about name and exec
    Ok(item)
}

#[cfg(target_os = "linux")]
pub fn load_items() -> Vec<Item> {
    let mut items = vec![];
    let files = load_desktop_file_list().unwrap_or(vec![]);
    for file in files {
        let mut should_continue = false;
        let item = parce_desktop_file(file).unwrap_or_else(|_| {
            should_continue = true;
            Item::default()
        });
        if !should_continue {
            items.push(item);
        }
    }
    items
}

#[cfg(target_os = "windows")]
pub fn load_items() -> Vec<Item> {
    vec![]
}
#[cfg(target_os = "linux")]
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

#[cfg(target_os = "windows")]
pub fn launch_app(app: &Item) {}

pub fn search_item(app_list: Vec<Item>, search_pattern: String) -> Vec<Item> {
    app_list
        .iter()
        .filter(|item| item.name.starts_with(&search_pattern))
        .cloned()
        .collect()
}
