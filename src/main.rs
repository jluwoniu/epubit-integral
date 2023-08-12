use std::error::Error;

use clap::{Parser, Subcommand};
use epubit_integral::{add_account, run};
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
    /// 添加账号
    Add {
        /// 账号
        #[arg[short, long]]
        username: String,
        /// 密码
        #[arg[short, long]]
        password: String,
        /// 开始页码
        #[arg[short='n', long]]
        page_number: Option<usize>,
    },
    /// 运行积分任务
    Run,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();
    match &cli.command {
        Commands::Add {
            username,
            password,
            page_number,
        } => {
            add_account(username, password, page_number)?;
        }
        Commands::Run => {
            run().await?;
        }
    }
    Ok(())
}
