pub mod error;
mod config;
mod build;
mod parse;
mod compile;
mod template;
mod pages;

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use clap::Parser;
use log::{debug, warn};
use tokio::task::JoinSet;

pub use error::*;
use crate::{
    build::build,
    build::ARGS,
    config::Config,
    config::Args,
    build::CONFIG
};
use crate::config::Page;
use crate::pages::list_pages;

pub static SITE_ROOT: OnceLock<Arc<PathBuf>> = OnceLock::new();

#[tokio::main]
pub async fn main() -> Result<()> {
    env_logger::init();

    let args = Arc::new(Args::parse());
    let config = args.config.as_ref().map(|i| i.clone()).unwrap_or(args.root.join("site.toml"));

    SITE_ROOT.set(Arc::new(config.clone())).expect("Failed to set site root");

    let config = Arc::new(toml::de::from_str::<Config>(&tokio::fs::read_to_string(config).await?)?);

    ARGS.set(Arc::clone(&args)).expect("Failed to set args");
    CONFIG.set(Arc::clone(&config)).expect("Failed to set config");

    let mut set = JoinSet::<Result<()>>::new();

    for page in list_pages().await? {
        debug!("{:#?}", &page);
    }

    while let Some(result) = set.join_next().await {
        result??;
    }

    Ok(())
}