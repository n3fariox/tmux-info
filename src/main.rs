mod hostname;
mod ip;
mod user;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tmux-info", about = "Cross-platform tmux statusline info")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the current username
    User,
    /// Print the short hostname
    Hostname,
    /// Print local IP addresses
    Ip,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::User => print!("{}", user::get_username()),
        Commands::Hostname => print!("{}", hostname::get_hostname()),
        Commands::Ip => print!("{}", ip::get_ips()),
    }
}
