use std::io::prelude::*;
use std::fs::File;

use anyhow::Result;

use serde::Deserialize;
use toml;

use crate::network::NetworkConfig;

const FILENAME: &str = "./wallet.toml";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bitcoind: NetworkConfig,
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
