use std::env;
use bdk_starter_example as lib;

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args[1] == "help" {
        cmd_help(&args[0]);
    } else if args[1] == "addr" {
        lib::cmd_addresses()?
    } else if args[1] == "newaddr" {
        lib::cmd_newaddr()?
    } else if args[1] == "tx" {
        lib::cmd_tx(&args[2..])?
    } else if args[1] == "spend" {
        lib::cmd_spend(&args[2..])?
    } else {
        eprintln!("invalid options: {}", args[1]);
        cmd_help(&args[0]);
        return Err(1);
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
    println!("                    Decode transaction hex string.");
    println!("  help              Show this help message and exit.");
}
