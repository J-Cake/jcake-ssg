use std::sync::Arc;
use std::sync::OnceLock;
use log::{debug, info};
use tokio::task::JoinSet;
use crate::{
    config::Args,
    config::Config,
    config::LanguageConfig,
    error::*,
    parse::ParsingContext,
    template
};
use crate::config::Page;

pub static ARGS: OnceLock<Arc<Args>> = OnceLock::new();
pub static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

pub async fn build(page: Page) -> Result<()> {
    let args = Arc::clone(ARGS.get().expect("No args set"));
    let config = Arc::clone(CONFIG.get().expect("No config set"));

    debug!("Building {:#?}", &page);

    Ok(())
}

// pub async fn build(language: LanguageConfig) -> Result<()> {
//     let args = Arc::clone(ARGS.get().expect("No args set"));
//
//     let mut set = JoinSet::new();
//
//     for file in language.pages.iter().map(|i| if i.is_absolute() { i.clone() } else { args.root.join(i) }) {
//         for file in globwalk::glob(file.to_string_lossy())? {
//             match file {
//                 Ok(file) if file.metadata()?.is_dir() => return Err(BuildError::MatchedDirectory(file.path().to_path_buf()).into()),
//                 Err(err) => return Err(Error::from(err)),
//                 Ok(file) => {
//                     let file = file.path().to_path_buf();
//
//                     info!("Building page {:?}", file);
//
//                     let lang = language.clone();
//                     set.spawn(async move {
//                         let source = tokio::fs::read_to_string(file.clone()).await?;
//                         let cx = ParsingContext::new(source, file.clone())?;
//
//                         // let template = TemplateContext {
//                         //     page: file.clone(),
//                         //     parse: cx,
//                         //     variables: vec![
//                         //         ("page".to_owned(), rune::to_value(file.clone().to_str().unwrap_or(""))?),
//                         //         ("language".to_owned(), rune::to_value(lang.clone())?)
//                         //     ].into_iter().collect(),
//                         // };
//
//                         Result::<()>::Ok(())
//                     });
//                 }
//             }
//         }
//     }
//
//     while let Some(task) = set.join_next().await {
//         task??;
//     }
//
//     Ok(())
// }