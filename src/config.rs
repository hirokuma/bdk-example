use std::{fs::File, io::prelude::*};

use anyhow::Result;
use serde::Deserialize;
use toml;

use crate::network::{BitcoindConfig, ElectrumConfig, NetworkConfig};

const FILENAME: &str = "./wallet.toml";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub network: NetworkConfig,
    pub bitcoind: BitcoindConfig,
    pub electrum: ElectrumConfig,
}

impl Config {
    pub fn new() -> Result<Config> {
        let mut settings = String::new();
        let mut f = File::open(FILENAME)?;
        f.read_to_string(&mut settings)?;
        let data: Config = toml::from_str(&settings)?;
        Ok(data)
    }
}
