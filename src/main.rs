use std::env;
use std::process::Command;
use serde_json::json;
use url::{Url, form_urlencoded};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    let input: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    
    if input.trim().is_empty() || input.trim() == "install" {
        install_arcrunner();
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

fn install_arcrunner() {
    let mut should_relaunch = false;

    let localappdata = env::var("LOCALAPPDATA").expect("Failed to get LOCALAPPDATA environment variable");
    let flow_launcher_path = PathBuf::from(&localappdata).join("FlowLauncher/Flow.Launcher.exe");
    if !flow_launcher_path.exists() {
        eprintln!("Flow Launcher not found.");
        return;
    }

    if is_flow_launcher_running() {
        should_relaunch = true;
        println!("Closing Flow Launcher...");
        let _ = Command::new("taskkill")
            .args(&["/F", "/IM", "Flow.Launcher.exe"])
            .output()
            .expect("Failed to close Flow Launcher");
        thread::sleep(Duration::from_millis(50));
    }

    let exe_path = env::current_exe()
        .expect("Failed to get current executable path")
        .display()
        .to_string();

    let app_data_path = env::var("APPDATA").expect("Failed to get APPDATA environment variable");
    let flow_settings_path = PathBuf::from(&app_data_path).join("FlowLauncher/Settings/Settings.json");

    if flow_settings_path.exists() {
        let mut json_data: Value = serde_json::from_str(
            &fs::read_to_string(&flow_settings_path)
                .expect("Failed to read FlowLauncher's Settings.json"),
        )
        .expect("Failed to parse FlowLauncher's Settings.json");

        if let Some(custom_browser_list) = json_data
            .get_mut("CustomBrowserList")
            .and_then(|v| v.as_array_mut())
        {
            let arc_index = if let Some(index) = custom_browser_list.iter().position(|entry| {
                entry.get("Name")
                    .and_then(|name| name.as_str())
                    .map(|s| s == "Arc")
                    .unwrap_or(false)
            }) {
                let arc_entry = custom_browser_list.get_mut(index).unwrap();
                arc_entry["Path"] = json!(exe_path);
                index
            } else {
                let arc_entry = json!({
                    "Name": "Arc",
                    "Path": exe_path,
                    "PrivateArg": null,
                    "EnablePrivate": false,
                    "OpenInTab": true,
                    "Editable": true
                });
                custom_browser_list.push(arc_entry);
                custom_browser_list.len() - 1
            };

            json_data["CustomBrowserIndex"] = json!(arc_index);
        } else {
            eprintln!("Found FlowLauncher's Settings.json, but could not locate the CustomBrowserList.");
        }

        fs::write(
            &flow_settings_path,
            serde_json::to_string_pretty(&json_data)
                .expect("Failed to serialize modified FlowLauncher Settings JSON"),
        )
        .expect("Failed to update FlowLauncher's Settings.json");

        println!("FlowLauncher's Settings.json updated successfully!");
    } else {
        eprintln!("Flow Launcher's Settings.json not found at {}.", flow_settings_path.display());
    }

    if should_relaunch {
        println!("Relaunching Flow Launcher...");
        Command::new(flow_launcher_path.to_str().unwrap())
            .spawn()
            .expect("Failed to relaunch Flow Launcher");
    }

    println!("ArcRunner install finished");
}

fn is_flow_launcher_running() -> bool {
    let output = Command::new("tasklist")
        .args(&["/FI", "IMAGENAME eq Flow.Launcher.exe"])
        .output()
        .expect("Failed to execute tasklist command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("Flow.Launcher.exe")
}
