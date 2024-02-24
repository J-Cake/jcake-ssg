use std::collections::HashMap;
use crate::error::*;
use crate::parse;
use crate::parse::Attribute;
use crate::parse::Element as ParseElement;
use crate::template::elements::Body;
use crate::template::elements::Element;

pub struct BlockElement {
    pub(super) source: parse::Element,
    pub(super) body: Vec<Body>,
    pub(super) attr: HashMap<String, Attribute>
}

impl Element for BlockElement {
    fn name(&self) -> String {
        self.source.name.clone()
    }

    fn render(&self, depth: u64) -> String {
        String::new()
    }
}