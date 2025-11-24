use anyhow::Result;
use bdk_wallet::{
    PersistedWallet, SignOptions,
    bitcoin::{Address, Amount, FeeRate, Transaction},
    rusqlite::Connection,
};

pub fn segwit_v1(
    wallet: &mut PersistedWallet<Connection>,
    // prev_tx: Transaction,
    // prev_index: u32,
    pay_addr: Address,
    pay_amount: Amount,
    fee_rate: FeeRate,
) -> Result<Transaction> {
    let mut psbt = {
        let mut builder = wallet.build_tx();
        builder.only_witness_utxo();
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
