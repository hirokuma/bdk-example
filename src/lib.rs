pub mod config;
pub mod network;
pub mod segwit;

use std::{
    str::FromStr, sync::{Arc, Mutex},
};

use anyhow::Result;
use bdk_wallet::{
    KeychainKind,
    bitcoin::{Address, Amount, FeeRate, Transaction, consensus::encode},
};
use tokio::sync::Notify;

use crate::{
    config::Config,
    network::{BitcoindRpc, ElectrumRpc, BackendRpc},
    segwit::{v1, wallet::MyWallet},
};

pub fn cmd_create(_: &Config) -> Result<()> {
    MyWallet::create_wallet()?;
    println!("Success.");
    Ok(())
}

pub fn cmd_addresses(config: &Config) -> Result<()> {
    let (wallet, _) = init(config)?;
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

pub fn cmd_newaddr(config: &Config) -> Result<()> {
    let (mut wallet, _) = init(config)?;
    let addr = wallet.wallet.next_unused_address(KeychainKind::External);
    wallet.persist();
    println!("address: {}", addr);
    Ok(())
}

pub fn cmd_tx(_: &Config, tx_hex: &String) -> Result<()> {
    let tx: Transaction = encode::deserialize_hex(tx_hex)?;
    println!("{:#?}", tx);
    Ok(())
}

pub fn cmd_spend(
    config: &Config,
    out_addr: &String,
    amount: u64,
    fee_rate: f64,
) -> Result<()> {
    let out_addr = receivers_address(out_addr);
    let out_amount = Amount::from_sat(amount);
    let fee_rate = FeeRate::from_sat_per_kwu((fee_rate * 1000.0 / 4.0) as u64);

    let (mut wallet, _) = init(config)?;
    let tx = v1::segwit_v1(
        &mut wallet.wallet,
        out_addr,
        out_amount,
        fee_rate,
    )?;

    let s: Vec<u8> = encode::serialize(&tx);
    let hex_str = s.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex_str);
    println!("vsize: {}", tx.vsize());
    Ok(())
}

pub fn cmd_sendtx(
    config: &Config,
    hex: &String,
) -> Result<()> {
    let (_, rpc) = init(config)?;
    let tx: Transaction = encode::deserialize_hex(hex)?;
    let txid = rpc.send_rawtx(&tx)?;
    println!("txid: {}", txid);
    Ok(())
}

pub async fn cmd_stay(config: &Config) -> Result<()> {
    let (wallet, rpc) = init(config)?;
    let wallet = Arc::new(Mutex::new(wallet));

    // sync_loop(wallet, rpc).await;
    infinite_loop().await;
    Ok(())
}

async fn sync_loop(wallet: Arc<Mutex<MyWallet>>, rpc: Arc<Mutex<dyn BackendRpc>>) {
    loop {
        {
            let g = rpc.lock().unwrap();
            let mut w = wallet.lock().unwrap();
            g.sync(&mut w).unwrap();
        }
    }
}

fn init(config: &Config) -> Result<(MyWallet, Box<dyn BackendRpc>)> {
    let mut wallet = MyWallet::load_wallet()?;
    let rpc: Box<dyn BackendRpc> = match &*config.network.backend {
        "bitcoind" => Box::new(BitcoindRpc::new(&config.bitcoind)?),
        "electrum" => Box::new(ElectrumRpc::new(&config.electrum)?),
        _ => anyhow::bail!("unknown network: {}", config.network.backend),
    };

    {
        rpc.full_scan(&mut wallet)?;
    }
    Ok((wallet, rpc))
}

fn receivers_address(addr: &str) -> Address {
    Address::from_str(addr)
        .expect("a valid address")
        .require_network(MyWallet::WALLET_NETWORK)
        .expect("valid address for mainnet")
}

async fn infinite_loop() {
    let notify = Arc::new(Notify::new());
    notify.notified().await;
}
