// https://deepwiki.com/search/fullscan-sync_eb8b6154-2c2f-467f-a300-15bb525c492d?mode=fast

use std::sync::Arc;

use anyhow::Result;

use serde::Deserialize;

use bdk_bitcoind_rpc::{
    Emitter, NO_EXPECTED_MEMPOOL_TXS,
    bitcoincore_rpc::{Auth, Client, RpcApi},
};

use bdk_wallet::{
    Balance,
    bitcoin::{Transaction, Txid},
    chain::local_chain::CheckPoint,
};

use crate::segwit::wallet::MyWallet;
use bdk_electrum::{BdkElectrumClient, electrum_client};

pub trait BackendRpc {
    fn full_scan(&self, wallet: &mut MyWallet) -> Result<()>;
    fn sync(&self, wallet: &mut MyWallet) -> Result<()>;
    fn send_rawtx(&self, tx: &Transaction) -> Result<Txid>;
}

#[derive(Deserialize, Debug)]
pub struct NetworkConfig {
    pub backend: String,
}

#[derive(Deserialize, Debug)]
pub struct BitcoindConfig {
    pub user: String,
    pub password: String,
    pub server: String,
}

pub struct BitcoindRpc {
    client: Client,
}

impl BitcoindRpc {
    pub fn new(config: &BitcoindConfig) -> Result<BitcoindRpc> {
        let client: Client = Client::new(
            &config.server,
            Auth::UserPass(config.user.clone(), config.password.clone()),
        )?;
        Ok(BitcoindRpc { client })
    }
}

// https://github.com/bitcoindevkit/bdk/blob/4fe121e7167cf93a8abf26c87d35b26a682f6cbc/examples/example_bitcoind_rpc_polling/src/main.rs
impl BackendRpc for BitcoindRpc {
    fn full_scan(&self, wallet: &mut MyWallet) -> Result<()> {
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
            wallet.wallet.apply_block_connected_to(
                &block.block,
                block.block_height(),
                block.connected_to(),
            )?;
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

    fn sync(&self, wallet: &mut MyWallet) -> Result<()> {
        self.full_scan(wallet)
    }

    fn send_rawtx(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self.client.send_raw_transaction(tx)?;
        Ok(txid)
    }
}

#[derive(Deserialize, Debug)]
pub struct ElectrumConfig {
    pub server: String,
}

pub struct ElectrumRpc {
    client: BdkElectrumClient<electrum_client::Client>,
}

impl ElectrumRpc {
    const BATCH_SIZE: usize = 5;
    const GAP_LIMIT: usize = 20;

    pub fn new(config: &ElectrumConfig) -> Result<ElectrumRpc> {
        let client = electrum_client::Client::new(&config.server)?;
        let client = BdkElectrumClient::new(client);
        Ok(ElectrumRpc { client })
    }
}

impl BackendRpc for ElectrumRpc {
    fn full_scan(&self, wallet: &mut MyWallet) -> Result<()> {
        let req = wallet.wallet.start_full_scan();
        let update =
            self.client
                .full_scan(req, ElectrumRpc::GAP_LIMIT, ElectrumRpc::BATCH_SIZE, true)?;
        wallet.wallet.apply_update(update)?;

        Ok(())
    }

    fn sync(&self, wallet: &mut MyWallet) -> Result<()> {
        let req = wallet.wallet.start_sync_with_revealed_spks();
        let update = self.client.sync(req, ElectrumRpc::BATCH_SIZE, true)?;
        wallet.wallet.apply_update(update)?;

        Ok(())
    }

    fn send_rawtx(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self.client.transaction_broadcast(tx)?;
        Ok(txid)
    }
}
