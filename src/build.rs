use std::sync::{Arc, OnceLock};
use tokio::task::JoinSet;
use crate::config::Args;
use crate::config::Config;
use crate::config::LanguageConfig;
use crate::error::*;
use crate::template::PageFactory;

pub static ARGS: OnceLock<Arc<Args>> = OnceLock::new();
pub static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

pub async fn build(language: LanguageConfig) -> Result<()> {
    let args = Arc::clone(ARGS.get().expect("No args set"));
    // let config = Arc::clone(CONFIG.get().expect("No config set"));

    let mut set = JoinSet::new();

    for file in language.pages.iter().map(|i| if i.is_absolute() { i.clone() } else { args.root.join(i) }) {
        for file in globwalk::glob(file.to_string_lossy())? {
            match file {
                Ok(file) if file.metadata()?.is_dir() => return Err(BuildError::MatchedDirectory(file.path().to_path_buf()).into()),
                Ok(file) => { set.spawn(PageFactory::new(tokio::fs::read_to_string(file.path().to_path_buf()).await?, file.path().to_path_buf())); },
                Err(err) => return Err(Error::from(err))
            }
        }
    }

    while let Some(task) = set.join_next().await {
        dbg!(task??);
    }

    Ok(())
}