use std::io::prelude::*;
use std::sync::Arc;
use std::fs::File;

use anyhow::Result;

use serde::Deserialize;
use toml;

use bdk_bitcoind_rpc::{
    Emitter, NO_EXPECTED_MEMPOOL_TXS,
    bitcoincore_rpc::{Auth, Client, RpcApi},
};

use bdk_wallet::{Balance, bitcoin::Transaction, chain::local_chain::CheckPoint};

use crate::segwit::wallet::MyWallet;

const FILENAME: &str = "./wallet.toml";

#[derive(Deserialize, Debug)]
pub struct BitcoinRpc {
    pub user: String,
    pub password: String,
    pub server: String,
}

impl BitcoinRpc {
    pub fn new() -> Result<BitcoinRpc> {
        let mut settings = String::new();
        let mut f = File::open(FILENAME)?;
        f.read_to_string(&mut settings)?;
        let data: BitcoinRpc = toml::from_str(&settings)?;
        Ok(data)
    }

    pub fn sync(&mut self, wallet: &mut MyWallet) -> Result<()> {
        let rpc_client: Client = Client::new(
            &self.server,
            Auth::UserPass(self.user.clone(), self.password.clone()),
        )?;

        let blockchain_info = rpc_client.get_blockchain_info()?;
        println!(
            "\nConnected to Bitcoin Core RPC.\nChain: {}\nLatest block: {} at height {}\n",
            blockchain_info.chain, blockchain_info.best_block_hash, blockchain_info.blocks,
        );

        let wallet_tip: CheckPoint = wallet.wallet.latest_checkpoint();
        println!(
            "Current wallet tip is: {} at height {}",
            &wallet_tip.hash(),
            &wallet_tip.height()
        );

        let mut emitter = Emitter::new(
            &rpc_client,
            wallet_tip.clone(),
            wallet_tip.height(),
            NO_EXPECTED_MEMPOOL_TXS,
        );

        println!("Syncing blocks...");
        while let Some(block) = emitter.next_block()? {
            print!("{} ", block.block_height());
            wallet
                .wallet
                .apply_block_connected_to(&block.block, block.block_height(), block.connected_to())?;
        }
        println!();

        println!("Syncing mempool...");
        let mempool_emissions: Vec<(Arc<Transaction>, u64)> = emitter.mempool()?.update;
        wallet.wallet.apply_unconfirmed_txs(mempool_emissions);

        let balance: Balance = wallet.wallet.balance();
        println!("Wallet balance after syncing: {}", balance.total());
        Ok(())
    }
}
