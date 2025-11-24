use anyhow::Result;
use bdk_starter_example::{self as lib, config::Config};
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create wallet
    Create,
    /// Get addresses.
    Addr,
    /// Get new address.
    #[command(name = "newaddr")]
    NewAddr,
    /// Decode transaction hex string.
    Tx {
        /// hex string to decode
        hex: String,
    },
    /// Create a spendable transaction.
    Spend {
        /// output address
        out_addr: String,
        /// amount sats
        amount: u64,
        /// feerate
        fee_rate: f64,
    },
    #[command(name = "sendtx")]
    SendTx {
        /// hex string to sendrawtransaction
        hex: String,
    },
    /// Stay polling
    Stay,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::new()?;

    match cli.command {
        None => {
            // clap will show help if user asks, but when no subcommand provided, print help
            Cli::command().print_help()?;
            println!();
        }
        Some(Commands::Create) => lib::cmd_create(&config)?,
        Some(Commands::Addr) => lib::cmd_addresses(&config)?,
        Some(Commands::NewAddr) => lib::cmd_newaddr(&config)?,
        Some(Commands::Tx { hex }) => lib::cmd_tx(&config, &hex)?,
        Some(Commands::Spend {
            out_addr,
            amount,
            fee_rate,
        }) => lib::cmd_spend(&config, &out_addr, amount, fee_rate)?,
        Some(Commands::SendTx { hex }) => lib::cmd_sendtx(&config, &hex)?,
        Some(Commands::Stay) => lib::cmd_stay(&config).await?,
    }

    Ok(())
}
