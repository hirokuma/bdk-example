pub mod network;
pub mod segwit;

use std::str::FromStr;

use anyhow::Result;

use bdk_wallet::KeychainKind;
use bitcoin::{Address, Amount, FeeRate, Transaction, consensus::encode};
use segwit::{v1, wallet::MyWallet};

use network::BitcoinRpc;

pub fn cmd_addresses() -> Result<()> {
    let (wallet, _) = init()?;
    let index = match wallet.wallet.derivation_index(KeychainKind::External) {
        None => { return Err(anyhow::anyhow!("No addresses found")); },
        Some(index) => index,
    };
    for i in 0..=index {
        let addr = wallet.wallet.peek_address(KeychainKind::External, i);
        println!("{}: {}", i, addr);
    }
    Ok(())
}

pub fn cmd_newaddr() -> Result<()> {
    let (mut wallet, _) = init()?;
    let addr = wallet.wallet.next_unused_address(KeychainKind::External);
    wallet.persist();
    println!("address: {}", addr);
    Ok(())
}

pub fn cmd_tx(tx_hex: &String) -> Result<()> {
    let tx: Transaction = encode::deserialize_hex(tx_hex)?;
    println!("{:#?}", tx);
    Ok(())
}

pub fn cmd_spend(
    prev_tx_hex: &String,
    prev_index: u32,
    out_addr: &String,
    amount: u64,
    fee_rate: f64,
) -> Result<()> {
    let prev_tx: Transaction = encode::deserialize_hex(prev_tx_hex)?;
    let out_addr = receivers_address(out_addr);
    let out_amount = Amount::from_sat(amount);
    let fee_rate = FeeRate::from_sat_per_kwu((fee_rate * 1000.0 / 4.0) as u64);

    let (mut wallet, _) = init()?;
    let tx = v1::segwit_v1(
        &mut wallet.wallet,
        prev_tx,
        prev_index,
        out_addr,
        out_amount,
        fee_rate,
    )?;

    let s: Vec<u8> = encode::serialize(&tx);
    let hex_str = s.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex_str);
    println!("{:#?}", tx);
    println!("vsize: {}", tx.vsize());
    Ok(())
}

fn init() -> Result<(MyWallet, BitcoinRpc)> {
    let mut wallet = MyWallet::create_wallet()?;
    let mut rpc = BitcoinRpc::new()?;
    rpc.sync(&mut wallet);

    Ok((wallet, rpc))
}

fn receivers_address(addr: &str) -> Address {
    Address::from_str(addr)
        .expect("a valid address")
        .require_network(MyWallet::WALLET_NETWORK)
        .expect("valid address for mainnet")
}
