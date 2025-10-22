use std::sync::Arc;

use anyhow::Result;

use serde::Deserialize;

use bdk_bitcoind_rpc::{
    Emitter, NO_EXPECTED_MEMPOOL_TXS,
    bitcoincore_rpc::{Auth, Client, RpcApi},
};

use bdk_wallet::{
    bitcoin::{Transaction, Txid},
    chain::local_chain::CheckPoint, Balance,
};

use crate::segwit::wallet::MyWallet;

#[derive(Deserialize, Debug)]
pub struct NetworkConfig {
    pub user: String,
    pub password: String,
    pub server: String,
}

pub struct NetworkRpc {
    client: Client,
}

impl NetworkRpc {
    pub fn new(config: &NetworkConfig) -> Result<NetworkRpc> {
        let client: Client = Client::new(
            &config.server,
            Auth::UserPass(config.user.clone(), config.password.clone()),
        )?;
        Ok(NetworkRpc{ client })
    }

    pub fn sync(&self, wallet: &mut MyWallet) -> Result<()> {
        let blockchain_info = self.client.get_blockchain_info()?;
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
            &self.client,
            wallet_tip.clone(),
            wallet_tip.height(),
            NO_EXPECTED_MEMPOOL_TXS,
        );

        print!("Syncing blocks...");
        while let Some(block) = emitter.next_block()? {
            wallet
                .wallet
                .apply_block_connected_to(&block.block, block.block_height(), block.connected_to())?;
        }
        println!("done.");

        println!("Syncing mempool...");
        let mempool_emissions: Vec<(Arc<Transaction>, u64)> = emitter.mempool()?.update;
        wallet.wallet.apply_unconfirmed_txs(mempool_emissions);
        wallet.persist();

        let balance: Balance = wallet.wallet.balance();
        println!("Wallet balance after syncing: {}", balance.total());
        Ok(())
    }

    pub fn send_rawtx(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self.client.send_raw_transaction(tx)?;
        Ok(txid)
    }
}
