use anyhow::Result;
use bdk_wallet::{KeychainKind, PersistedWallet, Wallet, rusqlite::Connection};
use bitcoin::Network;

const WALLET_XPRV_BASE: &str = "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu";
const WALLET_EXTR_PATH: &str = "86'/0'/0'/0/*";
const WALLET_INTR_PATH: &str = "86'/0'/0'/1/*";

pub fn create_wallet() -> Result<PersistedWallet<Connection>> {
    let mut db = Connection::open_in_memory().expect("Can't open database");

    let xprv_extn = format!("tr({}/{})", WALLET_XPRV_BASE, WALLET_EXTR_PATH);
    let xprv_intr = format!("tr({}/{})", WALLET_XPRV_BASE, WALLET_INTR_PATH);
    let wallet_opt = Wallet::load()
        .descriptor(KeychainKind::External, Some(xprv_extn.clone()))
        .descriptor(KeychainKind::Internal, Some(xprv_intr.clone()))
        .extract_keys()
        .check_network(Network::Bitcoin)
        .load_wallet(&mut db)?;
    match wallet_opt {
        Some(wallet) => Ok(wallet),
        None => {
            let wallet = Wallet::create(xprv_extn.clone(), xprv_intr.clone())
            .network(Network::Bitcoin)
            .create_wallet(&mut db)?;
            Ok(wallet)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bdk_wallet::AddressInfo;
    use bitcoin::bip32::{DerivationPath, Xpriv};
    use bitcoin::key::{TapTweak, XOnlyPublicKey};
    use bitcoin::secp256k1::{PublicKey, Secp256k1};
    use bitcoin::{Address, Network, Script};

    use super::*;

    #[test]
    // BIP-86 Test Vectors
    // https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki#test-vectors
    fn test_descriptor() {
        let mut db = Connection::open_in_memory().expect("Can't open database");

        // Account 0, first receiving address = m/86'/0'/0'/0/0
        let xprv1 = "tr(xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu/86'/0'/0'/0/*)";
        // Account 0, first change address = m/86'/0'/0'/1/0
        let xprv2 = "tr(xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu/86'/0'/0'/1/*)";
        let wallet_opt = Wallet::load()
            .descriptor(KeychainKind::External, Some(xprv1))
            .descriptor(KeychainKind::Internal, Some(xprv2))
            .extract_keys()
            .check_network(Network::Bitcoin)
            .load_wallet(&mut db)
            .expect("wallet");
        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => Wallet::create(xprv1, xprv2)
                .network(Network::Bitcoin)
                .create_wallet(&mut db)
                .expect("wallet"),
        };

        let address: AddressInfo = wallet.peek_address(KeychainKind::External, 0);
        assert_eq!(
            address.to_string(),
            "bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr",
            "external address"
        );
        println!(
            "Generated external address {} at index {}",
            address.address, address.index
        );
        let address: AddressInfo = wallet.peek_address(KeychainKind::Internal, 0);
        assert_eq!(
            address.to_string(),
            "bc1p3qkhfews2uk44qtvauqyr2ttdsw7svhkl9nkm9s9c3x4ax5h60wqwruhk7",
            "internal address"
        );
        println!(
            "Generated internal address {} at index {}",
            address.address, address.index
        );

        let secp = Secp256k1::new();
        let xprv = Xpriv::from_str("xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu").expect("Invalid xprv");
        let derivation_path = DerivationPath::from_str("m/86'/0'/0'/0/0").expect("Invalid path");
        let derived = xprv
            .derive_priv(&secp, &derivation_path)
            .expect("Derivation failed");
        let secret_key = derived.private_key;

        // 1. internal public key (untweaked)
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let xonly_pubkey = XOnlyPublicKey::from(public_key);
        assert_eq!(
            xonly_pubkey.to_string(),
            "cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115",
            "x-only pubkey"
        );
        println!("Internal x-only pubkey: {}", xonly_pubkey);

        // 2. tweaked pubkey
        let (tweaked_xonly, _parity) = xonly_pubkey.tap_tweak(&secp, None);
        assert_eq!(
            tweaked_xonly.to_string(),
            "a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c",
            "tweaked x-only pubkey"
        );
        println!("Tweaked x-only pubkey: {}", tweaked_xonly);

        // 3. scriptPubKey
        let mut script_bytes = Vec::with_capacity(1 + 32);
        script_bytes.push(0x51); // OP_1
        script_bytes.push(0x20); // length
        script_bytes.extend_from_slice(&tweaked_xonly.serialize());
        let script_pubkey = Script::from_bytes(&script_bytes);
        let script_pubkey_str = hex::encode(script_pubkey.as_bytes());
        assert_eq!(
            script_pubkey_str,
            "5120a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c",
            "scriptPubKey"
        );
        println!("scriptPubKey (hex): {}", script_pubkey_str);

        // 4. P2TR address
        let address = Address::p2tr_tweaked(tweaked_xonly, Network::Bitcoin);
        assert_eq!(
            address.to_string(),
            "bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr",
            "external address"
        );
        println!("P2TR address: {}", address);
    }
}
