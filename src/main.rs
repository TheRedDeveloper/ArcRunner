use std::env;
use std::process::Command;
use url::{Url, form_urlencoded};
use glob::glob;
use winreg::enums::*;
use winreg::RegKey;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn main() {
    let input: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    
    if input.trim() == "install" {
        install_arcrunner();
        return;
    }

    if input.trim().is_empty() {
        open_arc_browser();
        return;
    }

    let url = if input.starts_with('?') {
        let search_query = form_urlencoded::byte_serialize(input[1..].trim().as_bytes()).collect::<String>();
        format!("https://www.google.com/search?q={}", search_query)
    } else {
        input.trim_start_matches("--single-argument ").to_string()
    };

    if let Ok(parsed_url) = Url::parse(&url) {
        if parsed_url.scheme() == "http" || parsed_url.scheme() == "https" {
            Command::new("rundll32")
                .arg("url.dll,FileProtocolHandler")
                .arg(url)
                .spawn()
                .expect("Failed to open URL");
        } else {
            panic!("Invalid URL scheme: {}", parsed_url.scheme());
        }
    } else {
        panic!("Invalid URL format: {}", url);
    }
}

fn open_arc_browser() {
    let pattern = r"C:/Program Files/WindowsApps/TheBrowserCompany.Arc_*/Arc.exe";
    
    let mut found_path = None;
    
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if found_path.is_some() {
                    panic!("Multiple instances of Arc.exe found, can't decide which one to open.");
                }
                found_path = Some(path);
            },
            Err(e) => panic!("Error finding Arc.exe: {:?}", e),
        }
    }

    if let Some(arc_path) = found_path {
        Command::new(arc_path)
            .spawn()
            .expect("Failed to open Arc browser");
    } else {
        panic!("Arc.exe not found.");
    }
}

fn install_arcrunner() {
    let exe_path = env::current_exe()
        .expect("Failed to get current executable path")
        .display()
        .to_string();

    let mut search_shortcuts_path = PathBuf::from(env::var("LOCALAPPDATA").expect("Failed to get LOCALAPPDATA"));
    search_shortcuts_path.push("Microsoft/PowerToys/PowerToys Run/Settings/Plugins/Community.PowerToys.Run.Plugin.WebSearchShortcut/WebSearchShortcutStorage.json");

    if search_shortcuts_path.exists() {
        let mut json_data: Value = serde_json::from_str(
            &fs::read_to_string(&search_shortcuts_path).expect("Failed to read WebSearchShortcutStorage.json"),
        ).expect("Failed to parse JSON");
        if let Value::Object(ref mut map) = json_data {
            for (_, value) in map.iter_mut() {
                if let Value::Object(ref mut search_shortcut) = value {
                    search_shortcut.insert("BrowserPath".to_string(), Value::String(exe_path.clone()));
                } else {
                    panic!("Unexpected JSON format in WebSearchShortcutStorage.json");
                }
            }
        } else {
            panic!("Unexpected JSON format in WebSearchShortcutStorage.json");
        }
        fs::write(
            &search_shortcuts_path,
            serde_json::to_string_pretty(&json_data).expect("Failed to serialize modified JSON"),
        ).expect("Failed to write updated WebSearchShortcutStorage.json");
        println!("WebSearchShortcutStorage.json updated successfully.");
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let user_choice_key = hkcu
        .open_subkey("Software\\Microsoft\\Windows\\Shell\\Associations\\UrlAssociations\\http\\UserChoice")
        .expect("Failed to open UserChoice registry key");

    let prog_id: String = user_choice_key
        .get_value("ProgId")
        .expect("Failed to read ProgId value");

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let shell_open_command_key_path = format!("{}\\shell\\open\\command", prog_id);
    let shell_open_command_key = hkcr
        .open_subkey_with_flags(&shell_open_command_key_path, KEY_WRITE)
        .expect("Failed to open shell\\open\\command registry key");

    shell_open_command_key
        .set_value("", &exe_path)
        .expect("Failed to set shell\\open\\command (Default) value");
    
    println!("ArcRunner probably installed successfully.");
}