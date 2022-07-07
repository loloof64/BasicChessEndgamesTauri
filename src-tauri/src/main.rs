#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::{thread, time};

use serde::{Deserialize, Serialize};

use dirs::config_dir;
use std::fs::{self, OpenOptions};
use std::sync::Mutex;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct Settings {
    engine_path: String,
}

impl Settings {
    fn new() -> Self {
        Self {
            engine_path: "".to_string(),
        }
    }
}

fn settings_path() -> Result<String, String> {
    match config_dir() {
        Some(mut path_buf) => {
            path_buf.push("Basic Chess Endgames");
            match fs::create_dir_all(path_buf.as_path()) {
                Err(reason) => println!("Failed to create settings directory : {}", reason),
                _ => {}
            }

            path_buf.push("settings");
            path_buf.set_extension("json");
            match OpenOptions::new()
                .write(true)
                .create(true)
                .open(path_buf.as_path())
            {
                Err(reason) => return Err(format!("Failed to get settings path: {:?}", reason)),
                _ => {}
            }

            match path_buf.into_os_string().into_string() {
                Ok(path) => Ok(path),
                Err(reason) => Err(format!("Failed to get settings path : {:?}", reason)),
            }
        }
        _ => Err("Failed to get settings path.".to_string()),
    }
}

fn load_settings_from_file() -> Result<Settings, String> {
    match settings_path() {
        Ok(path) => match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<Settings>(&content) {
                Ok(configuration) => Ok(configuration),
                Err(reason) => Err(format!(
                    "Failed to parse content from configuration file : {}",
                    reason
                )),
            },
            _ => Err("Failed to read content from configuration file.".to_string()),
        },
        Err(reason) => Err(reason),
    }
}

fn save_settings_to_file(settings: &Settings) -> Result<(), String> {
    match settings_path() {
        Ok(settings_path) => match serde_json::to_string_pretty(settings) {
            Ok(content) => match fs::write(settings_path, content) {
                Ok(()) => Ok(()),
                Err(reason) => Err(format!(
                    "Failed to save configuration values in configuration file: {}",
                    reason
                )),
            },
            _ => Err("Failed to convert configuration values to string.".to_string()),
        },
        Err(reason) => Err(reason),
    }
}

#[tauri::command]
fn get_settings(settings: tauri::State<Mutex<Settings>>) -> Result<String, String> {
    match serde_json::to_string_pretty(&settings.inner()) {
        Ok(content) => Ok(content),
        Err(reason) => Err(format!(
            "Failed to convert configuration values to string : {}",
            reason
        )),
    }
}

#[tauri::command]
fn save_settings(settings: tauri::State<Mutex<Settings>>, settings_json: String) -> Result<(), String> {
    match serde_json::from_str::<Settings>(&settings_json) {
        Ok(parsed_content) => {
            match save_settings_to_file(&parsed_content) {
                Ok(()) => match settings.lock() {
                    Ok(mut settings_locked) => {
                        *settings_locked = parsed_content;
                        Ok(())
                    },
                    _ => Err("Failed to get lock from settings state.".to_string())
                },
                Err(reason) => Err(format!("Failed to save settings to file : {}", reason))
            }
        }
        Err(reason) => Err(format!("Failed to parse settings from string : {}", reason)),
    }
}

#[tauri::command]
fn check_uci_engine_path(absolute_path: String) -> Result<(), String> {
    let path = absolute_path.clone();
    let child = Command::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(mut child) = child {
        if let Some(stdout) = child.stdout.take() {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(b"uci\n");

                thread::spawn(move || {
                    let delay = time::Duration::from_secs_f32(8.0);
                    thread::sleep(delay);

                    let _ = child.kill();
                });

                thread::spawn(move || {
                    let stdout_reader = BufReader::new(stdout);
                    let stdout_lines = stdout_reader.lines();

                    for line in stdout_lines {
                        if let Ok(line) = line {
                            if line.trim().contains("uciok") {
                                let _ = stdin.write_all(b"quit\n");

                                let delay = time::Duration::from_millis(300);
                                thread::sleep(delay);

                                return Ok(());
                            }
                        }
                    }

                    Err(format!(
                        "The selected program is not an UCI engine : {}",
                        absolute_path
                    ))
                })
                .join()
                .expect("Failed to launch stdout reader thread.")
            } else {
                return Err(format!(
                    "The selected program has no stdin : {}",
                    absolute_path
                ));
            }
        } else {
            return Err(format!(
                "The selected program has no stdout : {}",
                absolute_path
            ));
        }
    } else {
        return Err(format!(
            "The selected program is not a valid program or has too restricted access rights : {}",
            absolute_path
        ));
    }
}

fn main() {
    let settings = match load_settings_from_file() {
        Ok(settings) => settings,
        Err(reason) => {
            println!("{}", reason);
            Settings::new()
        }
    };

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(Mutex::new(settings))
        .invoke_handler(tauri::generate_handler![
            check_uci_engine_path,
            get_settings,
            save_settings
        ])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
