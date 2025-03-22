use crate::{ssh, util};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process;

#[derive(Parser)]
#[command(name = "keyport")]
#[command(about = "Utility tool which simplifies working with openssh cli tools")]
pub struct Interface {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Add an existing key")]
    Add { name: String },

    #[command(about = "Remove an added key")]
    Remove { name: String },
}

impl Interface {
    pub fn run(&self) {
        let ssh_dir = ssh::directory().expect("failed to find ssh directory");

        match &self.command {
            Command::Add { name } => Interface::add(&ssh_dir.join(name)),
            Command::Remove { name } => Interface::remove(&ssh_dir.join(name)),
        }
    }

    fn add(filename: &Path) {
        util::create_file_with_comment(filename, "# Paste your private key here")
            .expect("failed to create file");

        util::launch_editor(filename).expect("failed to launch editor");
        util::set_file_permissions(filename, "600").expect("failed to set permissions");

        let pub_file = filename.with_extension("pub");
        util::create_file_with_comment(&pub_file, "# Paste your public key here")
            .expect("failed to create pub file");

        util::launch_editor(&pub_file).expect("failed to launch editor");
        util::set_file_permissions(&pub_file, "600").expect("failed to set permissions");

        ssh::ensure_agent_running().expect("failed to start ssh-agent");

        process::Command::new("ssh-add")
            .arg(filename)
            .output()
            .expect("failed to ssh-add key");
    }

    fn remove(filename: &Path) {
        ssh::ensure_agent_running().expect("failed to start ssh-agent");

        process::Command::new("ssh-add")
            .arg("-d")
            .arg(filename)
            .output()
            .expect("");

        fs::remove_file(filename).expect("failed to remove private key file");
        fs::remove_file(filename.with_extension("pub")).expect("failed to remove public key file");
    }
}
