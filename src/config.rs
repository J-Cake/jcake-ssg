use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser;
use rune::{Any};
use serde::Deserialize;
use serde::Serialize;

#[inline]
fn default_language() -> String { "en".into() }

#[inline]
fn default_build() -> PathBuf { "build".into() }

#[inline]
fn default_content_type() -> Vec<Arc<ContentType>> {
    vec![]
}

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(long, short)]
    pub config: Option<PathBuf>,

    #[clap(long, short, default_value = "./")]
    pub root: PathBuf,

    #[clap(long = "language", short, num_args(0..))]
    pub languages: Vec<String>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub url: Option<String>,
    #[serde(default = "default_language")]
    pub default_language: String,

    #[serde(rename = "language")]
    pub languages: Vec<Arc<LanguageConfig>>,

    #[serde(rename = "page")]
    pub pages: Vec<Page>,

    pub roots: Vec<PathBuf>,

    #[serde(default = "default_build")]
    pub build: PathBuf,

    #[serde(rename = "content-type", default = "default_content_type")]
    pub content_types: Vec<Arc<ContentType>>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Any)]
pub struct LanguageConfig {
    #[serde(rename = "abbreviation")]
    pub name: String,

    #[serde(rename = "full-name")]
    pub native: String,

    pub menu: Vec<Menu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Menu {
    Submenu {
        label: String,
        items: Vec<Menu>
    },
    Item {
        label: String,
        page: String
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentType {
    pub extensions: Vec<String>,
    pub handler: String // A Rune script
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub name: String,
    pub title: Option<String>,
}