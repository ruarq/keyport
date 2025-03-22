use clap::Parser;
use keyport::cli::Interface;

fn main() {
    Interface::parse().run();
}
