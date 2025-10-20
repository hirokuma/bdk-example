use anyhow::Result;

use bdk_wallet::{
    rusqlite::Connection, KeychainKind, PersistedWallet, SignOptions,
    bitcoin::{
        psbt::Input, Address, Amount, FeeRate, OutPoint, Transaction,
    },
};

pub fn segwit_v1(
    wallet: &mut PersistedWallet<Connection>, 
    prev_tx: Transaction,
    prev_index: u32,
    pay_addr: Address,
    pay_amount: Amount,
    fee_rate: FeeRate,
) -> Result<Transaction> {
    println!("signers len: {}", wallet.get_signers(KeychainKind::External).signers().len());

    let prev_outpoint = OutPoint {
        txid: prev_tx.compute_txid(),
        vout: prev_index,
    };
    let input = Input {
        witness_utxo: Some(prev_tx.tx_out(prev_index as usize)?.clone()),
        ..Default::default()
    };
    let weight = wallet.public_descriptor(KeychainKind::External).max_weight_to_satisfy()?;
    let mut psbt = {
        let mut builder = wallet.build_tx();
        builder.only_witness_utxo();
        builder.add_foreign_utxo(prev_outpoint, input, weight)?;
        builder.add_recipient(pay_addr.script_pubkey(), pay_amount);
        builder.fee_rate(fee_rate);
        builder.finish()?
    };
    let b = wallet.sign(
        &mut psbt, 
        SignOptions {
            trust_witness_utxo: true,
            ..Default::default()
        },
    )?;
    if !b {
        return Err(anyhow::Error::msg("sign error"));
    }
    Ok(psbt.extract_tx()?)
}
