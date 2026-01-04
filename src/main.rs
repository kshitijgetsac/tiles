use clap::{Args, Parser, Subcommand};
use std::error::Error;
mod commands;

/// Default number of relay attempts for model conversations
pub const DEFAULT_RELAY_COUNT: u32 = 10;

#[derive(Debug, Parser)]
#[command(name = "tiles")]
#[command(version, about = "Run, fine-tune models locally with Modelfile", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs the given Modelfile (runs the default model if none passed)
    Run {
        modelfile_path: Option<String>,

        /// Maximum number of relay attempts for model conversations
        #[arg(long, short = 'r', default_value_t = DEFAULT_RELAY_COUNT)]
        relay_count: u32,
    },
    /// Checks the status of dependencies
    Health,
    /// start or stop the daemon server
    Server(ServerArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct ServerArgs {
    #[command(subcommand)]
    command: Option<ServerCommands>,
}

#[derive(Debug, Subcommand)]
enum ServerCommands {
    /// Start the py server as a daemon
    Start,
    /// Stops the daemon py server
    Stop,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run {
            modelfile_path,
            relay_count,
        } => {
            commands::run(modelfile_path, relay_count).await;
        }
        Commands::Health => {
            commands::check_health();
        }
        Commands::Server(server) => match server.command {
            Some(ServerCommands::Start) => commands::start_server().await,
            Some(ServerCommands::Stop) => commands::stop_server(),
            _ => println!("Expected start or stop"),
        },
    }
    Ok(())
}
