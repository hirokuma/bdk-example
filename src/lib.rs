pub mod segwit;

use segwit::v1;
use bitcoin::consensus::encode;

pub fn cmd_addresses() -> Result<(), i32> {
    Err(1)
}

pub fn cmd_newaddr() -> Result<(), i32> {
    Err(1)
}

pub fn cmd_tx(args: &[String]) -> Result<(), i32> {
    println!("{:?}", args);
    Err(1)
}

pub fn cmd_spend(args: &[String]) -> Result<(), i32> {
    println!("{:?}", args);
    Err(1)
}

pub fn run_segwit_examples() {
    let tx = v1::segwit_v1().unwrap_or_else(|e| {
        eprintln!("Error creating segwit v1 transaction: {}", e);
        std::process::exit(1);
    });

    let s: Vec<u8> = encode::serialize(&tx);
    let hex_str = s.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex_str);
    println!("{:#?}", tx);
    println!("vsize: {}", tx.vsize());
}
