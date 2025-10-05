pub mod segwit;

use std::str::FromStr;

use bdk_wallet::{KeychainKind, PersistedWallet, rusqlite::Connection};
use bitcoin::{consensus::encode, Address, Amount, FeeRate, Transaction};
use segwit::{v1, wallet};

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

pub fn cmd_tx(tx_hex: &String) -> Result<(), String> {
    let tx: Transaction = match encode::deserialize_hex(tx_hex) {
        Ok(tx) => tx,
        Err(e) => { Err(e.to_string()) }?,
    };
    println!("{:#?}", tx);
    Ok(())
}

pub fn cmd_spend(
    prev_tx_hex: &String,
    prev_index: u32,
    out_addr: &String,
    amount: u64,
    fee_rate: f64
) -> Result<(), String> {
    let prev_tx: Transaction = match encode::deserialize_hex(prev_tx_hex) {
        Ok(tx) => tx,
        Err(e) => { Err(e.to_string()) }?,
    };
    let out_addr = receivers_address(out_addr);
    let out_amount = Amount::from_sat(amount);
    let fee_rate = FeeRate::from_sat_per_kwu((fee_rate * 4.0) as u64);

    println!("out_index: {}", prev_index);
    println!("out_addr: {}", out_addr);
    println!("amount: {}", amount);
    println!("fee_rate: {}", fee_rate);

    let mut wallet = init()?;
    let tx = match v1::segwit_v1(
        &mut wallet, 
        prev_tx, 
        prev_index,
        out_addr,
        out_amount,
        fee_rate,
    ) {
        Ok(tx) => tx,
        Err(e) => { Err(e.to_string()) }?,
    };

    let s: Vec<u8> = encode::serialize(&tx);
    let hex_str = s.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex_str);
    println!("{:#?}", tx);
    println!("vsize: {}", tx.vsize());
    Ok(())
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

fn receivers_address(addr: &str) -> Address {
    Address::from_str(addr)
        .expect("a valid address")
        .require_network(wallet::WALLET_NETWORK)
        .expect("valid address for mainnet")
}
