//! # CLI Parser
//! This module implements the parser for the command line interface.

use clap::{Parser, Subcommand};
use keyport::{ssh, util};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Clap parser for CLI arguments.
#[derive(Parser)]
#[command(name = "keyport", version, about)]
pub struct Interface {
    /// The subcommand to run.
    #[command(subcommand)]
    command: Command,
}

/// The possible commands.
#[derive(Subcommand)]
enum Command {
    #[command(about = "Add an existing key")]
    Add {
        #[arg(
            value_name = "FILE",
            help = "Filepath to a private key or name of a file in ~/.ssh"
        )]
        file: String,

        #[arg(long)]
        editor: Option<String>,

        #[arg(long)]
        no_password: bool,
    },

    #[command(about = "Remove an added key")]
    Remove {
        #[arg(
            value_name = "FILE",
            help = "Filepath to a private key or name of a file in ~/.ssh"
        )]
        file: String,
    },
    #[command(about = "Show a public key")]
    Show {
        #[arg(
            value_name = "FILE",
            help = "Filepath to a private key or name of a file in ~/.ssh"
        )]
        file: String,
    },

    #[command(name = "set-password", about = "Set the password of a key")]
    SetPassword {
        #[arg(
            value_name = "FILE",
            help = "Filepath to a private key or name of a file in ~/.ssh"
        )]
        file: String,
    },
}

impl Interface {
    /// Run the interface (after `Interface::parse()` has been called).
    pub fn run(&self) {
        match &self.command {
            Command::Add {
                file,
                editor,
                no_password,
            } => Interface::add(&Self::make_filepath(file), editor.as_deref(), *no_password),
            Command::Remove { file } => Interface::remove(&Self::make_filepath(file)),
            Command::Show { file } => Interface::show(&Self::make_filepath(file)),
            Command::SetPassword { file } => Interface::set_password(&Self::make_filepath(file)),
        }
    }

    fn make_filepath(file: &str) -> PathBuf {
        let path = Path::new(file);

        if path.parent().is_some_and(|dir| dir.is_dir()) {
            PathBuf::from(path)
        } else {
            ssh::directory()
                .expect("failed to find ssh directory")
                .join(file)
        }
    }

    /// Run the `keyport add` command to add already generated keys.
    ///
    /// # Parameters
    /// - `filepath`: The path to where the private key file should be created.
    /// - `no_password`: Whether to not add a password to the key file or not.
    fn add(filepath: &Path, editor: Option<&str>, no_password: bool) {
        util::create_file_with_comment(filepath, "Paste your private key here")
            .expect("failed to create file");

        util::launch_editor(filepath, editor).expect("failed to launch editor");
        util::set_file_permissions(filepath, "600").expect("failed to set permissions");

        let pub_filepath = filepath.with_extension("pub");
        util::create_file_with_comment(&pub_filepath, "Paste your public key here")
            .expect("failed to create pub file");

        util::launch_editor(&pub_filepath, editor).expect("failed to launch editor");
        util::set_file_permissions(&pub_filepath, "600").expect("failed to set permissions");

        ssh::ensure_agent_running().expect("failed to start ssh-agent");

        ssh::add_key(filepath).expect("failed to add ssh-add key");

        if !no_password {
            ssh::set_password(filepath).expect("failed to set password");
        }
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
