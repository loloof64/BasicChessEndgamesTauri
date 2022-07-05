#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::{thread, time};

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
                    let delay = time::Duration::from_millis(2_000);
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
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_uci_engine_path])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
