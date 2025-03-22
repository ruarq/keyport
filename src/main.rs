use clap::{Parser, Subcommand};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

#[derive(Parser)]
#[command(name = "ssh-util")]
#[command(about = "Utility which makes working with ssh keys easier")]
struct Interface {
    #[command(subcommand)]
    command: Command,
}

impl Interface {
    fn create_file_with_comment<P: ?Sized + AsRef<Path>>(
        filename: &P,
        comment: &'static str,
    ) -> io::Result<()> {
        fs::File::create_new(filename)?.write_all(comment.as_bytes())
    }

    fn set_key_file_permissions<P: ?Sized + AsRef<OsStr>>(
        filename: &P,
    ) -> io::Result<process::Output> {
        process::Command::new("chmod")
            .arg("600")
            .arg(filename)
            .output()
    }

    fn launch_editor<P: ?Sized + AsRef<OsStr>>(filename: &P) -> io::Result<process::ExitStatus> {
        process::Command::new("vim").arg(filename).status()
    }

    fn add(filename: &Path) {
        Self::create_file_with_comment(&filename, "# Paste your private key here")
            .expect("failed to create file");

        Self::launch_editor(filename).expect("failed to launch editor");
        Self::set_key_file_permissions(filename).expect("failed to set permissions");

        let pub_file = &filename.with_extension("pub");
        Self::create_file_with_comment(pub_file, "# Paste your public key here")
            .expect("failed to create pub file");

        Self::launch_editor(pub_file).expect("failed to launch editor");
        Self::set_key_file_permissions(pub_file).expect("failed to set permissions");

        process::Command::new("$(eval ssh-agent -s)")
            .output()
            .expect("failed to start ssh-agent");

        process::Command::new("ssh-add")
            .arg(filename)
            .output()
            .expect("failed to ssh-add key");
    }

    fn remove(filename: &Path) {
        process::Command::new("ssh-add")
            .arg("-d")
            .arg(filename)
            .output()
            .expect("");

        fs::remove_file(filename).expect("failed to remove private key file");
        fs::remove_file(filename.with_extension("pub")).expect("failed to remove public key file");
    }
}

#[derive(Subcommand)]
enum Command {
    Add { name: String },
    Remove { name: String },
}

fn main() {
    let interface = Interface::parse();

    let home_dir = std::env::var("HOME").expect("failed to get $HOME");
    let ssh_dir = Path::new(&home_dir).join(".ssh");

    match interface.command {
        Command::Add { name } => Interface::add(&ssh_dir.join(&name)),
        Command::Remove { name } => Interface::remove(&ssh_dir.join(&name)),
    }
}
