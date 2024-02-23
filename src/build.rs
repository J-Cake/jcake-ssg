use std::sync::Arc;
use std::sync::OnceLock;
use std::path::PathBuf;
use log::{debug, warn};
use crate::{
    parse::ParsingContext,
    error::*,
    config::Config,
    config::Args,
    config::ContentType,
    config::LanguageConfig,
    error::*,
    template::elements::Element
};

#[derive(Debug)]
pub struct PageResolver {
    pub path: PathBuf,
    pub language: Arc<LanguageConfig>,
    pub content_type: Arc<ContentType>
}


pub static ARGS: OnceLock<Arc<Args>> = OnceLock::new();
pub static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

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

pub async fn build(page: PageResolver) -> Result<()> {
    let source = tokio::fs::read_to_string(&page.path).await?;

    let mut cx = ParsingContext::new(source, page.path.clone())?;
    let page = cx.parse()?;
    let el = Element::from_element(&page)?;

    debug!("Transform: {:?}", el.render(0));

    Ok(())
}