use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

pub fn create_file_with_comment(filepath: &Path, comment: &str) -> io::Result<()> {
    fs::File::create_new(filepath)?.write_all(comment.as_bytes())
}

pub fn launch_editor(filepath: &Path) -> io::Result<process::ExitStatus> {
    process::Command::new("vim").arg(filepath).status()
}

pub fn set_file_permissions(filepath: &Path, permissions: &str) -> io::Result<process::Output> {
    process::Command::new("chmod")
        .arg(permissions)
        .arg(filepath)
        .output()
}
