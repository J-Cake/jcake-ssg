use std::path::PathBuf;
use std::sync::Arc;
use log::{debug, warn};
use crate::build::{ARGS, CONFIG};
use crate::config::{ContentType, LanguageConfig};
use crate::error::*;

#[derive(Debug)]
pub struct PageResolver {
    path: PathBuf,
    language: Arc<LanguageConfig>,
    content_type: Arc<ContentType>
}

pub async fn list_pages() -> Result<impl Iterator<Item=PageResolver>> {
    let args = ARGS.get().expect("Args not set").clone();
    let config = CONFIG.get().expect("Config not set").clone();

    let mut pages = Vec::new();

    for lang in args.languages.iter() {
        if let Some(language) = config.languages.iter().find(|i| i.name.eq(lang)) {
            for page in config.pages.iter() {
                for content_type in config.content_types.iter() {
                    for ext in content_type.extensions.iter() {
                        let mut potential = Vec::with_capacity(10);

                        for i in config.roots.iter()
                            .map(|root| Result::<PathBuf>::Ok(args.root
                                .canonicalize()?
                                .join(root)
                                .join(&page.name)
                                .with_extension(format!("{}.{}", &language.name, ext)))) {

                            let page = i?;
                            if tokio::fs::try_exists(&page).await? {
                                potential.push(page)
                            }
                        }

                        if potential.len() > 1 {
                            warn!("Ambiguous page name: {:?}", &potential);
                        } else if let Some(first) = potential.first() {
                            pages.push(PageResolver {
                                content_type: content_type.clone(),
                                language: language.clone(),
                                path: first.clone()
                            });
                        }
                    }
                }
            }
        } else {
            warn!("No language '{}' defined", lang);
        }
    }

    Ok(pages.into_iter())
}