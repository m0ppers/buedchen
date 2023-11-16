use std::{
    io::{self},
    process::{Command, ExitStatus, Stdio},
    thread::{self, JoinHandle},
};

pub enum ClientStartError {
    NoCommandGiven,
    SpawnError(io::Error),
}

impl From<io::Error> for ClientStartError {
    fn from(value: io::Error) -> Self {
        ClientStartError::SpawnError(value)
    }
}

impl std::fmt::Display for ClientStartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientStartError::NoCommandGiven => write!(f, "The supplied command was empty"),
            ClientStartError::SpawnError(e) => write!(f, "Couldn't spawn child process: {}", e),
        }
    }
}

pub fn run_client(
    args: &[String],
    socket_name: &str,
) -> Result<JoinHandle<Result<ExitStatus, io::Error>>, ClientStartError> {
    let mut args_iter = args.iter();
    let command = args_iter
        .next()
        .ok_or_else(|| ClientStartError::NoCommandGiven)?;
    let mut child = Command::new(command)
        .args(args_iter)
        .env("WAYLAND_DISPLAY", socket_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Spawn a thread to wait for the child process to exit
    Ok(thread::spawn(move || child.wait()))
}
