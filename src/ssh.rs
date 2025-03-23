use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

/// Get the `~/.ssh` directory.
pub fn directory() -> Option<PathBuf> {
    if let Ok(home) = env::var("HOME") {
        Some(PathBuf::from(home).join(".ssh"))
    } else {
        None
    }
}

/// Check if the `ssh-agent` is running.
pub fn is_agent_running() -> bool {
    if let Ok(auth_sock) = env::var("SSH_AUTH_SOCK") {
        fs::exists(auth_sock).is_ok()
    } else {
        false
    }
}

/// Start the `ssh-agent`.
pub fn start_agent() -> io::Result<process::Output> {
    process::Command::new("ssh-agent").arg("-s").output()
}

/// Starts the `ssh-agent` if it is not running.
pub fn ensure_agent_running() -> io::Result<()> {
    if !is_agent_running() {
        match start_agent() {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    } else {
        Ok(())
    }
}
