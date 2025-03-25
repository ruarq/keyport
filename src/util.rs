use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

/// Create a file and write `# {comment}` to it.
///
/// # Parameters
/// - `filepath`: The path to where the file should be created.
/// - `comment`: The comment to write to the file.
pub fn create_file_with_comment(filepath: &Path, comment: &str) -> io::Result<()> {
    write!(fs::File::create(filepath)?, "# {}", comment)
}

/// Get the preferred editor of the user.
pub fn get_env_editor() -> Option<String> {
    match env::var("EDITOR") {
        Ok(editor) => match editor.as_str() {
            // Reject nano, I don't understand why but for some reason it messes up the key files.
            "nano" => None,
            _ => Some(editor),
        },
        Err(_) => None,
    }
}

/// Launch the an editor for a file.
///
/// # Parameters
/// - `filepath`: The path to the file that should be opened in the editor.
/// - `editor`: The editor to prefer over $EDITOR.
pub fn launch_editor(filepath: &Path, editor: Option<&str>) -> io::Result<process::ExitStatus> {
    if let Some(editor) = editor {
        process::Command::new(editor).arg(filepath).status()
    } else if let Some(editor) = get_env_editor() {
        process::Command::new(editor).arg(filepath).status()
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "No editor, specify with option --editor.",
        ))
    }
}

/// Set the permissions of a file. Simply runs `$ chmod {permissions} {filepath}`.
///
/// # Parameters
/// - `filepath`: The path to the file to adjust the permissions of.
/// - `permissions`: String describing the permissions to set.
pub fn set_file_permissions(filepath: &Path, permissions: &str) -> io::Result<process::Output> {
    process::Command::new("chmod")
        .arg(permissions)
        .arg(filepath)
        .output()
}
