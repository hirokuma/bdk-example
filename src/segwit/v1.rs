use anyhow::Result;

use bdk_wallet::{AddressInfo, KeychainKind, SignOptions};
use bitcoin::{hashes::Hash, psbt::Input, Amount, FeeRate, OutPoint, Transaction, Txid};

use super::common;
use common::{ADDRESS_OUT_V1, DUMMY_UTXO_AMOUNT, SPEND_AMOUNT};
use super::wallet;

fn dummy_unspent_transaction_output(addr: AddressInfo, amount: Amount) -> (OutPoint, Input) {
    let outpoint = OutPoint {
        txid: Txid::all_zeros(),
        vout: 0,
    };
    let dummy_input = Input {
        witness_utxo: Some(bitcoin::TxOut {
            value: amount,
            script_pubkey: addr.script_pubkey(),
        }),
        ..Default::default()
    };
    (outpoint, dummy_input)
}

pub fn segwit_v1() -> Result<Transaction> {
    let mut wallet = wallet::create_wallet()?;
    let recv_addr = common::receivers_address(ADDRESS_OUT_V1);
    let prev_addr = wallet.next_unused_address(KeychainKind::External);
    let dummy_out_point = dummy_unspent_transaction_output(prev_addr, DUMMY_UTXO_AMOUNT);
    let weight = wallet.public_descriptor(KeychainKind::External).max_weight_to_satisfy()?;

    let mut psbt = {
        let mut builder = wallet.build_tx();
        builder.add_foreign_utxo(dummy_out_point.0, dummy_out_point.1, weight)?;
        builder.add_recipient(recv_addr.script_pubkey(), SPEND_AMOUNT);
        // builder.fee_absolute(DUMMY_UTXO_AMOUNT- (SPEND_AMOUNT + CHANGE_AMOUNT));
        builder.fee_rate(FeeRate::from_sat_per_vb_unchecked(1));
        builder.finish()?
    };
    wallet.sign(&mut psbt, SignOptions::default())?;
    Ok(psbt.extract_tx()?)
}
