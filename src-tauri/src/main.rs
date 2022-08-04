#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use engine_communication::UciEngine;
use serde::{Deserialize, Serialize};

use dirs::config_dir;
use std::fs::{self, OpenOptions};
use std::sync::Mutex;

mod engine_communication;

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
    engine_communication::check_uci_engine_path(absolute_path)
}

#[tauri::command]
fn execute_engine(engine: tauri::State<Mutex<UciEngine>>, engine_absolute_path: String) -> Result<(), String> {
    match engine.lock() {
        Ok(mut engine_locked) => engine_locked.execute(engine_absolute_path),
        _ => Err("Failed to get lock from engine.".to_string())
    }
}

#[tauri::command]
fn read_from_engine_outputs(engine: tauri::State<Mutex<UciEngine>>) -> Result<Option<String>, String> {
    match engine.lock() {
        Ok(engine_locked) => {
            Ok(engine_locked.read_from_outputs())
        }
        _ => Err("Failed to get lock from engine.".to_string())
    }
}

#[tauri::command]
fn send_command_to_engine(engine: tauri::State<Mutex<UciEngine>>, command: String) -> Result<(), String> {
    match  engine.lock() {
        Ok(engine_locked) => {
            engine_locked.send_command(command);
            Ok(())
        },
        _ => Err("Failed to get lock from engine.".to_string())
    }
}

#[tauri::command]
fn close_engine(engine: tauri::State<Mutex<UciEngine>>) -> Result<(), String> {
    match engine.lock() {
        Ok(engine_locked) => {
            engine_locked.close();
            Ok(())
        },
        _ => Err("Failed to get lock from engine.".to_string())
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

    let engine = engine_communication::UciEngine::new();

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(Mutex::new(settings))
        .manage(Mutex::new(engine))
        .invoke_handler(tauri::generate_handler![
            check_uci_engine_path,
            execute_engine,
            read_from_engine_outputs,
            send_command_to_engine,
            close_engine,
            get_settings,
            save_settings
        ])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("Error while running tauri application.");
}
