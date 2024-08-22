use crate::config::TASK;
use std::error::Error;
use std::process::Command;

pub async fn exec(task: &str) -> Result<String, Box<dyn Error>> {
    if !TASK.contains_key(task) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Task '{}' not found.", task),
        )))
    };
    let sh = TASK.get(task).unwrap();
    let mut command = None;
    let mut split = sh.split_whitespace();
    if let Some(first_part) = split.next() {
        command = Some(Command::new(first_part));
    }

    if let Some(ref mut cmd) = command {
        for part in split {
            cmd.arg(part);
        }
    }

    match command {
        Some(mut cmd) => {
            let output = cmd.output().expect("Failed to execute command");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                Ok(format!("Output: {}", stdout))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Interrupted,
                    format!("Error: {}", stderr),
                )))
            }
        }
        None => {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No command was constructed",
            )))
        }
    }
}