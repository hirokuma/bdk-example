use anyhow::Result;
use clap::{Parser, Subcommand, CommandFactory};

use bdk_starter_example as lib;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Get addresses.
    Addr,
    /// Get new address.
    NewAddr,
    /// Decode transaction hex string.
    Tx {
        /// hex string to decode
        hex: String,
    },
    /// Create a spendable transaction.
    Spend {
        /// input tx hex
        raw_tx: String,
        /// out index
        out_index: u32,
        /// output address
        out_addr: String,
        /// amount sats
        amount: u64,
        /// feerate
        fee_rate: f64,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // clap will show help if user asks, but when no subcommand provided, print help
            Cli::command().print_help()?;
            println!();
        }
        Some(Commands::Addr) => lib::cmd_addresses()?,
        Some(Commands::NewAddr) => lib::cmd_newaddr()?,
        Some(Commands::Tx { hex }) => lib::cmd_tx(&hex)?,
        Some(Commands::Spend { raw_tx, out_index, out_addr, amount, fee_rate }) => {
            lib::cmd_spend(&raw_tx, out_index, &out_addr, amount, fee_rate)?
        }
    }

    Ok(())
}
