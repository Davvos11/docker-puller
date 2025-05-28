use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("Failed to execute: {command}: status {status}")]
    Command{command: String, status: i32},
    #[error("Failed to run command: {0}")]
    Io(#[from] std::io::Error),
}