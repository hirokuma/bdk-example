pub mod segwit;

use segwit::v1;
use bitcoin::consensus::encode;

pub fn run_segwit_examples() {
    let tx = v1::segwit_v1().unwrap_or_else(|e| {
        eprintln!("Error creating segwit v1 transaction: {}", e);
        std::process::exit(1);
    });

    let s: Vec<u8> = encode::serialize(&tx);
    println!("length: {}", s.len());
    let hex_str = s.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex_str);
    println!("{:#?}", tx);
    println!("vsize: {}", tx.vsize());
}
