use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

pub struct UciEngine {
    outputs_receiver: Option<Receiver<String>>,
    inputs_sender: Option<Sender<String>>,
}

impl UciEngine {
    pub fn new() -> Self {
        Self {
            outputs_receiver: None,
            inputs_sender: None,
        }
    }

    pub fn execute(&mut self, engine_absolute_path: String) -> Result<(), String> {
        let path = engine_absolute_path.clone();
        let child = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();
        if let Ok(mut child) = child {
            if let Some(stdout) = child.stdout.take() {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(b"uci\n");

                    let (sender_inputs, receiver_inputs) = channel::<String>();
                    let (sender_outputs, receiver_outputs) = channel::<String>();

                    self.outputs_receiver = Some(receiver_outputs);
                    self.inputs_sender = Some(sender_inputs);

                    thread::spawn(move || {
                        let stdout_reader = BufReader::new(stdout);
                        let stdout_lines = stdout_reader.lines();

                        for line in stdout_lines {
                            if let Ok(line) = line {
                                let line_copy = line.clone();
                                let _ = sender_outputs.send(line_copy);
                            }
                        }
                    });

                    let mut stdin_writer = BufWriter::new(stdin);
                    thread::spawn(move || loop {
                        let command = receiver_inputs
                            .recv()
                            .expect("Failed to read input from UCI engine.");
                        let final_command = format!("{}\n", command);
                        let final_command_buffer = final_command.as_bytes();
                        let _ = stdin_writer.write_all(final_command_buffer);
                    });

                    Ok(())
                } else {
                    return Err(format!(
                        "The selected program has no stdin : {}",
                        engine_absolute_path
                    ));
                }
            } else {
                return Err(format!(
                    "The selected program has no stdout : {}",
                    engine_absolute_path
                ));
            }
        } else {
            return Err(format!(
            "The selected program is not a valid program or has too restricted access rights : {}",
            engine_absolute_path
        ));
        }
    }

    pub fn read_from_outputs(&self) -> Option<String> {
        match &self.outputs_receiver {
            Some(receiver) => {
                let output = receiver.recv_timeout(Duration::from_millis(1));
                match output {
                    Ok(output) => Some(output),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn send_command(&self, command: String) {
        if let Some(inputs_sender) = &self.inputs_sender {
            let _ = inputs_sender.send(format!("{}\n", command));
        }
    }

    pub fn close(&self) {
        self.send_command("quit".to_string());
    }
}

pub fn check_uci_engine_path(absolute_path: String) -> Result<(), String> {
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
                    let delay = Duration::from_secs_f32(8.0);
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

                                let delay = Duration::from_millis(300);
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
