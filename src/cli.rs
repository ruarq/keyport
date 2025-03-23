//! # CLI Parser
//! This module implements the parser for the command line interface.

use crate::{ssh, util};
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Clap parser for CLI arguments.
#[derive(Parser)]
#[command(name = "keyport")]
#[command(about = "Utility tool which simplifies working with openssh cli tools")]
pub struct Interface {
    /// The subcommand to run.
    #[command(subcommand)]
    command: Command,
}

/// The possible commands.
#[derive(Subcommand)]
enum Command {
    #[command(about = "Add an existing key")]
    Add { name: String },

    #[command(about = "Remove an added key")]
    Remove { name: String },

    #[command(about = "Show a public key")]
    Show { name: String },

    #[command(name = "set-password", about = "Set the password of a key")]
    SetPassword { name: String },
}

impl Interface {
    /// Run the interface (after `Interface::parse()` has been called).
    pub fn run(&self) {
        let ssh_dir = ssh::directory().expect("failed to find ssh directory");

        match &self.command {
            Command::Add { name } => Interface::add(&ssh_dir.join(name)),
            Command::Remove { name } => Interface::remove(&ssh_dir.join(name)),
            Command::Show { name } => Interface::show(&ssh_dir.join(name)),
            Command::SetPassword { name } => Interface::set_password(&ssh_dir.join(name)),
        }
    }

    /// Run the `keyport add` command to add already generated keys.
    ///
    /// # Parameters
    /// - `filepath`: The path to where the private key file should be created.
    fn add(filepath: &Path) {
        let fallback_editor = "vi";

        util::create_file_with_comment(filepath, "Paste your private key here")
            .expect("failed to create file");

        util::launch_editor(filepath, fallback_editor).expect("failed to launch editor");
        util::set_file_permissions(filepath, "600").expect("failed to set permissions");

        let pub_filepath = filepath.with_extension("pub");
        util::create_file_with_comment(&pub_filepath, "Paste your public key here")
            .expect("failed to create pub file");

        util::launch_editor(&pub_filepath, fallback_editor).expect("failed to launch editor");
        util::set_file_permissions(&pub_filepath, "600").expect("failed to set permissions");

        ssh::ensure_agent_running().expect("failed to start ssh-agent");
        ssh::add_key(filepath).expect("failed to ssh-add key");

        Self::show(filepath);
    }

    /// Run the `keyport remove` command to remove existing keys.
    ///
    /// # Parameters
    /// - `filepath`: The path to the private key file.
    fn remove(filepath: &Path) {
        ssh::ensure_agent_running().expect("failed to start ssh-agent");

        ssh::remove_key(filepath).expect("");

        fs::remove_file(filepath).expect("failed to remove private key file");
        fs::remove_file(filepath.with_extension("pub")).expect("failed to remove public key file");
    }

    /// Run the "keyport show` command to print a public key.
    ///
    /// # Parameters
    /// - `filepath`: The path to the private key file.
    fn show(filepath: &Path) {
        ssh::ensure_agent_running().expect("failed to start ssh-agent");
        let output = ssh::get_public_key(filepath).expect("failed to obtain public key");

        io::stdout()
            .write_all(&output)
            .expect("failed to write public key to stdout");
    }

    /// Run the `keyport set-password` command to set the password of a key.
    ///
    /// # Parameters
    /// - `filepath`: The path to the private key file.
    fn set_password(filepath: &Path) {
        ssh::ensure_agent_running().expect("failed to start ssh-agent");
        ssh::set_password(filepath).expect("failed to set password");
    }
}
