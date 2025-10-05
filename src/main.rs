use std::env;
use bdk_starter_example as lib;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args[1] == "help" {
        cmd_help(&args[0]);
    } else if args[1] == "addr" {
        lib::cmd_addresses()?
    } else if args[1] == "newaddr" {
        lib::cmd_newaddr()?
    } else if args[1] == "tx" {
        lib::cmd_tx(&args[2])?
    } else if args[1] == "spend" && args.len() == 7 {
        let raw_tx = &args[2];
        let out_index: u32 = args[3].parse().map_err(|e| format!("Invalid out_index: {}", e))?;
        let out_addr = &args[4];
        let amount: u64 = args[5].parse().map_err(|e| format!("Invalid amount: {}", e))?;
        let fee_rate: f64 = args[6].parse().map_err(|e| format!("Invalid fee_rate: {}", e))?;
        lib::cmd_spend(raw_tx, out_index, out_addr, amount, fee_rate)?
    } else {
        eprintln!("invalid options: {}", args[1]);
        cmd_help(&args[0]);
        return Err("invalid option".to_string());
    }
    Ok(())
}

fn cmd_help(cmd: &String) {
    println!("Usage: {} [options]", cmd);
    println!("\nOptions:");
    println!("  addr              Get addresses.");
    println!("  newaddr           Get new address.");
    println!("  tx <hex_string>   Decode transaction hex string.");
    println!("  spend <input_hex> <out_index> <output_address> <amount_sats> <feerate>");
    println!("                    Create a spendable transaction.");
    println!("  help              Show this help message and exit.");
}
