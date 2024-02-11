use std::ops::Range;
use std::path::{Path, PathBuf};
use regex::Regex;
use crate::error::*;

#[derive(Debug)]
pub struct PageFactory {
    parsed: Element,
    origin: PathBuf
}

#[derive(Debug)]
pub enum Element {
    Element {
        name: String,
        attributes: Vec<Attribute>,
        body: Vec<Element>,
        origin: (PathBuf, usize)
    },
    Expression {
        body: String,
        origin: (PathBuf, usize)
    },
    String {
        body: String,
        origin: (PathBuf, usize)
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String, // Expression
    pub origin: (PathBuf, usize)
}

struct ParsingContext<Source: AsRef<str> + 'static, Origin: AsRef<Path> + 'static> {
    pub tag_name: Regex,
    pub css_selector_shorthand: Regex,
    pub attribute: Regex,
    pub open_or_closing_tag: Regex,
    pub combined: Regex,
    pub garbage: Regex,
    pub closing: Regex,
    pub source: Source,
    pub origin: Origin
}

impl PageFactory {
    pub async fn new<Source: AsRef<str> + 'static, Origin: AsRef<Path> + 'static>(source: Source, origin: Origin) -> Result<Self> {
        let origin_path = origin.as_ref().to_path_buf();
        let len = source.as_ref().len();

        let mut cx = ParsingContext {
            tag_name: Regex::new(r#"^[a-zA-Z][a-zA-Z0-9_-]*$"#)?,
            css_selector_shorthand: Regex::new(r#"^(?<class1>\.[a-zA-Z][a-zA-Z0-9_-]*)*(?<id>#[a-zA-Z][a-zA-Z0-9_-]*)?(?<class2>\.[a-zA-Z][a-zA-Z0-9_-]*)*$"#)?,
            attribute: Regex::new(r#"(?<name>\w+)(?:=\s*(?<double>"[^"]*")|(?<single>'[^']*'))?"#)?,
            open_or_closing_tag: Regex::new(r#"</?(?<tag>[a-zA-Z][a-zA-Z0-9_-]*)"#)?,
            combined: Regex::new(r#"^<\s*(?<tag>[a-zA-Z][a-zA-Z0-9_-]*)\s*(?<selector>(?:\.[a-zA-Z][a-zA-Z0-9_-]*)*(?:#[a-zA-Z][a-zA-Z0-9_-]*)?(?:\.[a-zA-Z][a-zA-Z0-9_-]*)*)(?<attributes>(?:\s+\w+(?:\s*=\s*("[^"]*")|('[^']*'))?)*)\s*/?>"#)?,
            closing: Regex::new(r#"</\s*[a-zA-Z][a-zA-Z0-9_-]*\s*>"#)?,
            garbage: Regex::new(r#"^(?:[\s\n]+|<!--.*?-->)"#)?,
            source, origin,
        };

        Ok(Self {
            origin: origin_path,
            parsed: Element::Element {
                body: Self::parse(&cx, 0usize..len, 0usize)?,
                name: "template".to_owned(),
                origin: (cx.origin.as_ref().to_path_buf(), 0),
                attributes: vec![]
            }
        })
    }
    fn parse<Source: AsRef<str> + 'static, Origin: AsRef<Path> + 'static>(cx: &ParsingContext<Source, Origin>, mut range: Range<usize>, depth: usize) -> Result<Vec<Element>> {
        let mut elements = vec![];

        while let Some(ignored) = cx.garbage.find(&cx.source.as_ref()[range.clone()]) {
            if ignored.len() <= 0 {
                break;
            }

            range.start += ignored.len();
        }

        let source = &cx.source.as_ref()[range.clone()];

        let captures = match cx.combined.captures(source) {
            Some(captures) => captures,
            None => return Ok(elements)
        };

        let open = captures.get(0).unwrap();

        let tag = captures.name("tag").ok_or(Error::BuildError(BuildError::NoTagName))?.as_str().to_owned();
        let mut attributes = if let Some(attr) = captures.name("attributes") {
            cx.attribute
                .captures_iter(attr.as_str())
                .filter_map(|i| {
                    let value = i.name("double").or_else(|| i.name("single"))?.as_str();
                    let name = i.name("name")?;

                    Some(Attribute {
                        name: name.as_str().to_owned(),
                        value: value[1..value.len() - 1].to_owned(),

                        origin: (cx.origin.as_ref().to_owned(), range.start + attr.start() + name.start() + 1),
                    })
                })
                .collect::<Vec<Attribute>>()
        } else {
            vec![]
        };

        (|| -> Option<()> {
            let capture = captures.name("selector")?;
            let selectors = cx.css_selector_shorthand.captures(capture.as_str())?;

            let classes1 = selectors.name("class1").map(|i| i.as_str().split("."));
            let classes2 = selectors.name("class2").map(|i| i.as_str().split("."));

            let class = Attribute {
                name: "class".to_string(),
                value: {
                    let mut class = Vec::new();
                    if let Some(classes) = classes1 { class.extend(classes); }
                    if let Some(classes) = classes2 { class.extend(classes); }
                    class.join(" ").trim().to_owned()
                },
                origin: (cx.origin.as_ref().to_path_buf(), range.start + capture.start()),
            };

            if class.value.trim().len() > 0 {
                attributes.push(class);
            }

            if let Some(id) = selectors.name("id") {
                attributes.push(Attribute {
                    name: "id".to_string(),
                    value: id.as_str()[1..].to_owned(),
                    origin: (cx.origin.as_ref().to_path_buf(), range.start + capture.start() + id.start())
                });
            }

            Some(())
        })().ok_or(Error::BuildError(BuildError::BadSelectorList))?;

        let close = (|cx: &ParsingContext<Source, Origin>| {
            // Find closing tag by counting opening and closing tags with the same name
            let mut open_count = 1i64;
            let source = &source[open.end()..];

            for tag in Regex::new(format!(r#"</?{}"#, &tag).as_str())?
                .find_iter(source) {

                open_count += if tag.as_str().starts_with("</") { -1 } else { 1 };

                if open_count <= 0 {
                    return cx.closing
                        .captures_at(source, tag.start())
                        .and_then(|i| i.iter().next())
                        .flatten()
                        .ok_or(Error::BuildError(BuildError::NoClosingTag))
                }
            }

            return Err(Error::BuildError(BuildError::NoClosingTag));
        })(&cx);

        let new_range = range.start + open.len()..range.start + open.end() + close?.start();
        // dbg!(&cx.source.as_ref()[new_range.clone()]);

        elements.push(Element::Element {
            attributes,
            name: tag,
            body: Self::parse(cx, new_range, depth + 1)?,
            origin: (cx.origin.as_ref().to_path_buf(), range.start)
        });

        return Ok(elements);
    }

}