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
    write!(fs::File::create_new(filepath)?, "# {}", comment)
}

/// Get the preferred editor of the user.
pub fn get_preferred_editor() -> Option<String> {
    match env::var("EDITOR") {
        Ok(editor) => Some(editor),
        Err(_) => None,
    }
}

/// Launch the users preferred editor for a file. The preferred editor is determined by the
/// environment variable `$EDITOR`. If that variable is not set, the "fallback" editor will be
/// used (under the assumption that it exists).
///
/// # Parameters
/// - `filepath`: The path to the file that should be opened in the editor.
/// - `fallback_editor`: The binary to use when the preferred editor couldn't be determined or
///   doesn't exist.
pub fn launch_editor(filepath: &Path, fallback_editor: &str) -> io::Result<process::ExitStatus> {
    if let Some(editor) = get_preferred_editor() {
        process::Command::new(editor).arg(filepath).status()
    } else {
        process::Command::new(fallback_editor)
            .arg(filepath)
            .status()
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
