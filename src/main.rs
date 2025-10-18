use std::env;

use anyhow::Result;

use bdk_starter_example as lib;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let prog = args.get(0).map(|s| s.as_str()).unwrap_or("prog");

    match args.get(1).map(|s| s.as_str()) {
        None | Some("help") => {
            cmd_help(prog);
        }
        Some("addr") => lib::cmd_addresses()?,
        Some("newaddr") => lib::cmd_newaddr()?,
        Some("tx") => {
            let hex = args
                .get(2)
                .ok_or(anyhow::anyhow!("tx requires <hex_string>"))?;
            lib::cmd_tx(hex)?
        }
        Some("spend") => {
            if args.len() == 7 {
                let raw_tx = &args[2];
                let out_index: u32 = args[3]
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid out_index: {}", e))?;
                let out_addr = &args[4];
                let amount: u64 = args[5]
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid amount: {}", e))?;
                let fee_rate: f64 = args[6]
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid fee_rate: {}", e))?;
                lib::cmd_spend(raw_tx, out_index, out_addr, amount, fee_rate)?
            } else {
                eprintln!("spend requires 5 arguments");
                cmd_help(prog);
                return Err(anyhow::anyhow!("invalid option"));
            }
        }
        Some(other) => {
            eprintln!("invalid options: {:?}", args);
            cmd_help(prog);
            return Err(anyhow::anyhow!("invalid option: {}", other));
        }
    }

    Ok(())
}

fn cmd_help(cmd: &str) {
    println!("Usage: {} [options]", cmd);
    println!("\nOptions:");
    println!("  addr              Get addresses.");
    println!("  newaddr           Get new address.");
    println!("  tx <hex_string>   Decode transaction hex string.");
    println!("  spend <input_hex> <out_index> <output_address> <amount_sats> <feerate>");
    println!("                    Create a spendable transaction.");
    println!("  help              Show this help message and exit.");
}
