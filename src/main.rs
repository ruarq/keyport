use clap::Parser;

mod cli;

fn main() {
    cli::Interface::parse().run();
}
