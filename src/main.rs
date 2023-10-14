use std::error::Error;

use clap::{Parser, Subcommand};
use epubit_integral::run;
#[derive(Parser)]
#[command(name = "epubit-integral")]
#[command(author = "jluwoniu<jluwoniu@outlook.com>")]
#[command(version = "1.0")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 运行积分任务
    Run,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();
    match &cli.command {
        Commands::Run => {
            run().await?;
        }
    }
    Ok(())
}
