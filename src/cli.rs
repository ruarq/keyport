//! # CLI Parser
//! This module implements the parser for the command line interface.

use clap::{Parser, Subcommand};
use keyport::{ssh, util};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

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
        if !ssh::are_required_tools_installed() {
            eprintln!("openssh tools missing, please install them.");
        }

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
        let private_keyfile =
            NamedTempFile::new().expect("failed to create temporary file for private key");
        let public_keyfile = NamedTempFile::new().expect("failed to create for public key");

        util::create_file_with_comment(private_keyfile.path(), "Paste your private key here")
            .expect("failed to create file");
        util::launch_editor(private_keyfile.path(), editor).expect("failed to launch editor");

        util::create_file_with_comment(public_keyfile.path(), "Paste your public key here")
            .expect("failed to create pub file");
        util::launch_editor(public_keyfile.path(), editor).expect("failed to launch editor");

        private_keyfile
            .persist_noclobber(filepath)
            .expect("failed to move private key");
        public_keyfile
            .persist_noclobber(filepath.with_extension("pub"))
            .expect("failed to move public key");

        util::set_file_permissions(filepath, "600").expect("failed to set permissions");
        util::set_file_permissions(&filepath.with_extension("pub"), "600")
            .expect("failed to set permissions");

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
