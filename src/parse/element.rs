use std::path::Path;
use regex::Regex;
use crate::{
    parse::Origin,
    parse::Element,
    parse::Attribute,
    Error,
    BuildError,
    parse::ParsingContext
};

impl<Source: AsRef<str> + 'static, File: AsRef<Path> + 'static> ParsingContext<Source, File> {
    pub(super) fn parse_tag(&mut self, depth: usize) -> crate::Result<Element> {
        let captures = match self.combined.captures(&self) {
            Some(captures) => captures,
            None => return Err(Error::BuildError(BuildError::NotATag))
        };

        let open = captures.get(0).unwrap();

        let tag = captures.name("tag").ok_or(Error::BuildError(BuildError::NoTagName))?.as_str().to_owned();
        let mut attributes = if let Some(attr) = captures.name("attributes") {
            self.attribute
                .captures_iter(attr.as_str())
                .filter_map(|i| {
                    let value = i.name("double").or_else(|| i.name("single"))?.as_str();
                    let name = i.name("name")?;

                    Some(Attribute {
                        name: name.as_str().to_owned(),
                        value: value[1..value.len() - 1].to_owned(),

                        origin: Origin {
                            source: self.origin.as_ref().to_path_buf(),
                            offset: self.range().start + attr.start() + name.start() + 1,
                            token_length: i.get(0).unwrap().len(),
                            depth,
                        },
                    })
                })
                .collect::<Vec<Attribute>>()
        } else {
            vec![]
        };

        (|| -> Option<()> {
            let capture = captures.name("selector")?;
            let selectors = self.css_selector_shorthand.captures(capture.as_str())?;

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
                origin: Origin {
                    source: self.origin.as_ref().to_path_buf(),
                    offset: self.range().start + capture.start(),
                    token_length: capture.len(),
                    depth,
                },
            };

            if class.value.trim().len() > 0 {
                attributes.push(class);
            }

            if let Some(id) = selectors.name("id") {
                attributes.push(Attribute {
                    name: "id".to_string(),
                    value: id.as_str()[1..].to_owned(),
                    origin: Origin {
                        source: self.origin.as_ref().to_path_buf(),
                        offset: self.range().start + capture.start() + id.start(),
                        token_length: id.len(),
                        depth,
                    },
                });
            }

            Some(())
        })().ok_or(Error::BuildError(BuildError::BadSelectorList))?;

        if open.as_str().ends_with("/>") {
            return Ok(Element {
                attributes,
                name: tag.to_lowercase(),
                origin: Origin {
                    token_length: open.as_str().len(),
                    source: self.path(),
                    offset: self.range().start,
                    depth
                },
                body: vec![]
            });
        }

        let close = (|cx: &ParsingContext<Source, File>| {
            // Find closing tag by counting opening and closing tags with the same name
            let mut open_count = 1i64;
            let source = &self[open.end()..];

            for tag in Regex::new(format!(r#"</?{}"#, &tag).as_str())?
                .find_iter(source) {
                open_count += if tag.as_str().starts_with("</") { -1 } else { 1 };

                if open_count <= 0 {
                    return cx.closing
                        .captures_at(source, tag.start())
                        .and_then(|i| i.iter().next())
                        .flatten()
                        .ok_or(Error::BuildError(BuildError::NoClosingTag));
                }
            }

            return Err(Error::BuildError(BuildError::NoClosingTag));
        })(&self)?;

        let new_range = self.range().start + open.len()..self.range().start + open.end() + close.start();

        return Ok(Element {
            attributes,
            name: tag.to_lowercase(),
            origin: Origin {
                token_length: open.as_str().len() + new_range.len() + close.as_str().len(),
                source: self.path(),
                offset: self.range().start,
                depth
            },
            body: self.parse_body(new_range, depth + 1)?
        });
    }
}