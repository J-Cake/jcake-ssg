use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;
use async_recursion::async_recursion;
use crate::{
    parse::Body,
    parse::Element,
    parse::ParsingContext,
    error::*,
    SITE_ROOT
};

pub struct TemplateContext<Source: AsRef<str> + 'static, Origin: AsRef<Path> + 'static> {
    pub page: PathBuf,
    pub parse: ParsingContext<Source, Origin>,

    pub variables: HashMap<String, rune::Value>
}

#[async_recursion]
pub async fn expand_template(page: Element) -> Result<Element> {
    let mut body = Vec::new();

    for i in page.body {
        body.extend(match i {
            Body::Expression(expr) => vec![Body::Expression(expr)],
            Body::Literal(lit) => vec![Body::Literal(lit)],
            Body::Element(el) => match el.name.as_str() {
                "block" => if let Some(parent) = el.attributes.iter().find(|attr| attr.name.eq("parent")) {
                    let path = resolve_path(&parent.value, el.origin.source.clone());

                    let source = tokio::fs::read_to_string(path.clone()).await?;
                    let mut cx = ParsingContext::new(source, path.clone())?;

                    let template = expand_template(cx.parse()?).await?; // TODO: Figure out how to invoke the template and pass the body down.

                    dbg!(template);

                    vec![]
                } else {
                    vec![]
                },
                "fragment" => el.body,
                // "template" => TemplateContext::new(el),
                _ => vec![],
            }
        });
    }

    return Ok(Element {
        origin: page.origin,
        name: page.name,
        attributes: page.attributes,
        body,
    })
}

fn resolve_path<Path: AsRef<str>>(path: Path, current_file: PathBuf) -> PathBuf {
    let site_root = SITE_ROOT.get()
        .expect("Failed to acquire site root")
        .parent()
        .expect("Invalid Site root")
        .to_path_buf();
    let mut path = path.as_ref().to_owned();

    assert!(site_root.is_absolute());

    if path.starts_with("#") {
        path = path.replacen("#", &format!("{}/", site_root.to_string_lossy()), 1);
    }

    let mut path = PathBuf::from(path);

    if path.is_relative() {
        path = current_file.join(path);
    }

    return path;
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::template::resolve_path;

    #[test]
    fn test_resolve() {
        let site_root = PathBuf::from("/home/jcake/Code/personal-website/site.toml");
        let current_file = PathBuf::from("/home/jcake/Code/personal-website/www/home.en.html");

        assert_eq!(resolve_path("#include/frame.html", current_file.clone()), PathBuf::from("/home/jcake/Code/personal-website/include/frame.html"));
        assert_eq!(resolve_path("./frame.html", current_file.clone()), PathBuf::from("/home/jcake/Code/personal-website/www/frame.html"));
        assert_eq!(resolve_path("frame.html", current_file.clone()), PathBuf::from("/home/jcake/Code/personal-website/www/frame.html"));
    }
}