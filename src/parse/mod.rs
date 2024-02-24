mod literal;
mod element;
mod expression;

use std::{
    collections::VecDeque,
    fmt::Debug,
    fmt::Formatter,
    ops::Deref,
    ops::Range,
    path::Path,
    path::PathBuf
};
use regex::Regex;
use crate::error::*;

#[derive(Clone)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub body: Vec<Body>,
    pub origin: Origin,
}

#[derive(Debug, Clone)]
pub struct Origin {
    pub source: PathBuf,
    pub offset: usize,
    pub depth: usize,
    pub token_length: usize
}

impl Origin {
    pub fn len(&self) -> usize {
        self.token_length
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

#[derive(Clone)]
pub struct Expression {
    pub body: String,
    pub origin: Origin,
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}}", "    ".repeat(self.origin.depth), &self.body)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Body {
    Element(Element),
    Expression(Expression),
    Literal(Literal),
}

impl Body {
    pub fn origin(&self) -> &Origin {
        match &self {
            Self::Element(el) => &el.origin,
            Self::Expression(expr) => &expr.origin,
            Self::Literal(lit) => &lit.origin,
        }
    }
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

#[derive(Debug, Clone)]
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
    range_stack: VecDeque<Range<usize>>
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

            range_stack: vec![0..source.as_ref().len()].into_iter().collect(),
            source,
            origin,
        })
    }

    pub fn path(&self) -> PathBuf {
        self.origin.as_ref().to_path_buf()
    }

    pub fn range(&self) -> &Range<usize> {
        self.range_stack.get(self.range_stack.len() - 1).unwrap()
    }

    pub fn range_mut(&mut self) -> &mut Range<usize> {
        self.range_stack.get_mut(self.range_stack.len() - 1).unwrap()
    }

    pub fn skip_whitespace(&mut self) -> &mut Self {
        while let Some(ignored) = self.garbage.find(&self) {
            if ignored.len() <= 0 { return self; }

            self.range_mut().start += ignored.len();
        }

        return self;
    }

    pub fn parse(&mut self) -> Result<Element> {
        Ok(Element {
            origin: Origin {
                depth: 0,
                offset: 0,
                source: self.path(),
                token_length: self.len(),
            },
            attributes: vec![Attribute {
                name: "origin".to_string(),
                value: self.path().to_str().unwrap().to_string(),
                origin: Origin {
                    depth: 0,
                    offset: 0,
                    source: self.path(),
                    token_length: self.len(),
                }
            }],
            name: "fragment".to_owned(),
            body: self.parse_body(self.range().clone(), 1)?,
        })
    }

    fn parse_body(&mut self, range: Range<usize>, depth: usize) -> Result<Vec<Body>> {
        let mut body = Vec::new();

        self.range_stack.push_back(range);

        while let Some(child) = self.try_parse_any(depth)? {
            self.range_mut().start += child.origin().len();
            body.push(child);
        }

        self.range_stack.pop_back().unwrap();

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
        &self.source.as_ref()[self.range().clone()]
    }
}