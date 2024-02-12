mod literal;
mod element;
mod expression;

use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::ops::{Deref, Range};
use std::path::{Path, PathBuf};
use regex::Regex;
use crate::error::*;

pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub body: Vec<Body>,
    pub origin: Origin,
}

#[derive(Debug)]
pub struct Origin {
    pub source: PathBuf,
    pub offset: usize,
    pub depth: usize,
}

impl Origin {
    pub fn from_path<Origin: AsRef<Path>>(origin: Origin, offset: usize, depth: usize) -> Self {
        Self {
            source: origin.as_ref().to_path_buf(),
            offset,
            depth,
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<{}", "    ".repeat(self.origin.depth), &self.name)?;
        for attr in self.attributes.iter() {
            write!(f, " {}=\"{}\"", &attr.name, &attr.value)?;
        }

        if self.body.len() > 0 {
            write!(f, ">")?;

            for i in self.body.iter() {
                f.write_str("\n")?;
                match i {
                    Body::Element(el) => el.fmt(f)?,
                    Body::Expression(expr) => expr.fmt(f)?,
                    Body::Literal(lit) => lit.fmt(f)?,
                };
            }

            write!(f, "\n{}</{}>", "    ".repeat(self.origin.depth), &self.name)
        } else {
            write!(f, " />")
        }
    }
}

pub struct Expression {
    pub body: String,
    pub origin: Origin,
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}}", "    ".repeat(self.origin.depth), &self.body)
    }
}

pub struct Literal {
    pub body: Vec<u8>,
    // To allow for byte strings
    pub origin: Origin,
    pub is_byte_string: bool,
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_byte_string {
            write!(f, r#"{}b"{:x?}""#, "    ".repeat(self.origin.depth), &self.body)
        } else {
            write!(f, r#"{}"{}""#, "    ".repeat(self.origin.depth), String::from_utf8(self.body.clone()).unwrap())
        }
    }
}

pub enum Body {
    Element(Element),
    Expression(Expression),
    Literal(Literal),
}

impl Debug for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(body) => body.fmt(f),
            Self::Expression(body) => body.fmt(f),
            Self::Literal(body) => body.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    // Expression
    pub origin: Origin,
}

#[derive(Clone)]
pub struct ParsingContext<Source: AsRef<str> + 'static, Origin: AsRef<Path> + 'static> {
    // HTML regex
    pub tag_name: Regex,
    pub css_selector_shorthand: Regex,
    pub attribute: Regex,
    pub open_or_closing_tag: Regex,
    pub combined: Regex,
    pub garbage: Regex,
    pub closing: Regex,

    // Literal regex
    pub legal_string_modifier: Regex,
    // pub escaped_string: Regex,

    source: Source,
    origin: Origin,
    range: Range<usize>,
}

impl<Source: AsRef<str> + 'static, File: AsRef<Path> + 'static> ParsingContext<Source, File> {
    pub fn new(source: Source, origin: File) -> Result<Self> {
        Ok(Self {
            tag_name: Regex::new(r#"^[a-zA-Z][a-zA-Z0-9_-]*$"#)?,
            css_selector_shorthand: Regex::new(r#"^(?<class1>\.[a-zA-Z][a-zA-Z0-9_-]*)*(?<id>#[a-zA-Z][a-zA-Z0-9_-]*)?(?<class2>\.[a-zA-Z][a-zA-Z0-9_-]*)*$"#)?,
            attribute: Regex::new(r#"(?<name>\w+)(?:=\s*(?<double>"[^"]*")|(?<single>'[^']*'))?"#)?,
            open_or_closing_tag: Regex::new(r#"</?(?<tag>[a-zA-Z][a-zA-Z0-9_-]*)"#)?,
            combined: Regex::new(r#"^<\s*(?<tag>[a-zA-Z][a-zA-Z0-9_-]*)\s*(?<selector>(?:\.[a-zA-Z][a-zA-Z0-9_-]*)*(?:#[a-zA-Z][a-zA-Z0-9_-]*)?(?:\.[a-zA-Z][a-zA-Z0-9_-]*)*)(?<attributes>(?:\s+\w+(?:\s*=\s*("[^"]*")|('[^']*'))?)*)\s*/?>"#)?,
            closing: Regex::new(r#"</\s*[a-zA-Z][a-zA-Z0-9_-]*\s*>"#)?,
            garbage: Regex::new(r#"^(?:[\s\n]+|<!--.*?-->)"#)?,

            legal_string_modifier: Regex::new(r#"^(?<mod>r?b?|b?r?)(?<hash>#{0,256})(?<quot>["'`])"#)?, // You would have to manually check that the hashtags always follow a modifier
            // escaped_string: ,

            range: 0..source.as_ref().len(),
            source,
            origin,
        })
    }
    pub fn range(&mut self, range: Range<usize>) -> &mut Self {
        self.range = range;
        return self;
    }

    pub fn origin(&self, depth: usize) -> Origin {
        Origin::from_path(self.origin.as_ref().to_path_buf(), self.range.start, depth)
    }

    pub fn skip_whitespace(&mut self) -> &mut Self {
        while let Some(ignored) = self.garbage.find(&self) {
            if ignored.len() <= 0 { return self; }

            self.range.start += ignored.len();
        }

        return self;
    }

    pub fn take(&mut self, chars: usize) -> String {
        if self.len() < chars {
            return String::new();
        }

        let str = self[..chars].to_owned();
        self.range.start += chars;
        return str;
    }

    pub fn parse(&mut self) -> Result<Element> {
        Ok(Element {
            origin: self.origin(0),
            attributes: vec![Attribute {
                name: "origin".to_string(),
                value: self.origin.as_ref().to_path_buf().to_str().unwrap().to_string(),
                origin: self.origin(0)
            }],
            name: "template".to_owned(),
            body: self.parse_body(1)?,
        })
    }

    fn parse_body(&mut self, depth: usize) -> Result<Vec<Body>> {
        let mut body = Vec::new();
        while let Some(child) = self.try_parse_any(depth)? {
            body.push(child);
        }

        Ok(body)
    }

    fn try_parse_any(&mut self, depth: usize) -> Result<Option<Body>> {
        self.skip_whitespace();

        match self.parse_expr(depth) {
            Ok(expr) => return Ok(Some(Body::Expression(expr))),
            Err(Error::BuildError(BuildError::NotAnExpression)) => (),
            Err(err) => return Err(err)
        }

        match self.parse_literal(depth) {
            Ok(lit) => return Ok(Some(Body::Literal(lit))),
            Err(Error::BuildError(BuildError::NotALiteral)) => (),
            Err(err) => return Err(err)
        }

        match self.parse_tag(depth) {
            Ok(tag) => return Ok(Some(Body::Element(tag))),
            Err(Error::BuildError(BuildError::NotATag)) => (),
            Err(err) => return Err(err)
        }

        return Ok(None);
    }
}

impl<Source: AsRef<str> + 'static, Origin: AsRef<Path> + 'static> Deref for ParsingContext<Source, Origin> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.source.as_ref()[self.range.clone()]
    }
}