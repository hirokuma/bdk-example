use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::{self, Builder};
use bdk_wallet::{bitcoin::Network, chain::spk_client::{FullScanRequestBuilder, FullScanResponse}, rusqlite::Connection, KeychainKind, Wallet};

const DB_PATH: &str = "./test_wallet.db";
const STOP_GAP: usize = 20;
const PARALLEL_REQUESTS: usize = 10;

fn main() {
    let tpub = "[12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T";
    let descriptor = format!("tr({}/0/*)#z3x5097m", tpub);
    let change_descriptor = format!("tr({}/1/*)#n9r4jswr", tpub);

    // Initiate the connection to the database
    let mut conn = Connection::open(DB_PATH).expect("Can't open database");

    // Create the wallet
    let external_descriptor = Some(descriptor.clone());
    let internal_descriptor = Some(change_descriptor.clone());
    let wallet_opt = Wallet::load()
        .descriptor(KeychainKind::External, external_descriptor)
        .descriptor(KeychainKind::Internal, internal_descriptor)
        // .extract_keys() // uncomment this line when using private descriptors
        .check_network(Network::Signet)
        .load_wallet(&mut conn)
        .unwrap();

    let mut wallet = if let Some(loaded_wallet) = wallet_opt {
        loaded_wallet
    } else {
        Wallet::create(descriptor.clone(), change_descriptor.clone())
            .network(Network::Signet)
            .create_wallet(&mut conn)
            .unwrap()
    };

    // Sync the wallet
    let client: esplora_client::BlockingClient =
        Builder::new("https://blockstream.info/signet/api/").build_blocking();

    println!("Syncing wallet...");
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResponse<KeychainKind> = client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .unwrap();

    // Apply the update from the full scan to the wallet
    wallet.apply_update(update).unwrap();

    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
}
