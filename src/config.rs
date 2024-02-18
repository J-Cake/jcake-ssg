use std::path::PathBuf;
use clap::Parser;
use rune::runtime::VmResult;
use rune::{Any, ToValue, Value};
use rune::ast::Kind::Struct;
use serde::Deserialize;
use serde::Serialize;

#[inline]
fn default_language() -> String { "en".into() }

#[inline]
fn default_build() -> PathBuf { "build".into() }

#[inline]
fn default_content_type() -> Vec<ContentType> {
    vec![]
}

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(long, short)]
    pub config: Option<PathBuf>,

    #[clap(long, short, default_value = "./")]
    pub root: PathBuf,

    #[clap(long, short)]
    pub languages: Vec<String>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub url: Option<String>,
    #[serde(default = "default_language")]
    pub default_language: String,

    #[serde(rename = "language")]
    pub languages: Vec<LanguageConfig>,

    #[serde(default = "default_build")]
    pub build: PathBuf,

    #[serde(rename = "content_type", default = "default_content_type")]
    pub content_types: Vec<ContentType>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Any)]
pub struct LanguageConfig {
    pub name: String,
    pub native: String,

    pub menu: Vec<(String, String)>,

    pub pages: Vec<PathBuf>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentType {
    pub extensions: Vec<String>,
    pub handler: String // A Rune script
}