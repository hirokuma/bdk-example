pub mod segwit;

use bdk_wallet::KeychainKind;
use bdk_wallet::PersistedWallet;
use bdk_wallet::rusqlite::Connection;
use bitcoin::consensus::encode;
use segwit::v1;
use segwit::wallet;

pub fn cmd_addresses() -> Result<(), String> {
    let wallet = init()?;
    let index = match wallet.derivation_index(KeychainKind::External) {
        Some(index) => index,
        None => Err("derivation not found")?,
    };
    for i in 0..index {
        let addr = wallet.peek_address(KeychainKind::External, i);
        println!("{}: {}", i, addr);
    }
    Ok(())
}

pub fn cmd_newaddr() -> Result<(), String> {
    let mut wallet = init()?;
    let addr = wallet.next_unused_address(KeychainKind::External);
    println!("address: {}", addr);
    Ok(())
}

pub fn cmd_tx(args: &[String]) -> Result<(), String> {
    println!("{:?}", args);
    Err("not implemented".to_string())
}

pub fn cmd_spend(args: &[String]) -> Result<(), String> {
    println!("{:?}", args);

    let tx = v1::segwit_v1().unwrap_or_else(|e| {
        eprintln!("Error creating segwit v1 transaction: {}", e);
        std::process::exit(1);
    });

    let s: Vec<u8> = encode::serialize(&tx);
    let hex_str = s.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex_str);
    println!("{:#?}", tx);
    println!("vsize: {}", tx.vsize());
    Err("not implemented".to_string())
}

fn init() -> Result<PersistedWallet<Connection>, String> {
    match wallet::create_wallet() {
        Ok(wallet) => Ok(wallet),
        Err(e) => {
            eprintln!("Error init: {}", e);
            Err(e.to_string())
        }
    }
}
