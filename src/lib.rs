use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use rcon::Builder;

pub mod cli;
pub mod commands;
pub mod configfile;
pub mod error;

#[derive(Clone, Serialize, Deserialize, ValueEnum)]
pub enum Quirk {
    MinecraftQuirk,
    FactorioQuirk,
}

impl Display for Quirk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quirk::MinecraftQuirk => f.write_fmt(format_args!("minecraft-quirk")),
            Quirk::FactorioQuirk => f.write_fmt(format_args!("factorio-quirk")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Server {
    name: String,
    address: String,
    password: String,
    quirks: Vec<Quirk>,
}

impl Server {
    pub fn enable_quirks<T>(&self, mut builder: Builder<T>) -> Builder<T> {
        for quirk in self.quirks.iter() {
            match quirk {
                Quirk::MinecraftQuirk => builder = builder.enable_minecraft_quirks(true),
                Quirk::FactorioQuirk => builder = builder.enable_factorio_quirks(true),
            }
        }
        builder
    }
}
