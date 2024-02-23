use std::collections::HashMap;
use crate::parse;
use crate::parse::Attribute;
use crate::template::elements::{Body, Element};

pub struct BlockElement {
    source: parse::Element,
    body: Vec<Body>,
    attr: HashMap<String, Attribute>
}

impl Element for BlockElement {
    fn name(&self) -> String {
        self.source.name.clone()
    }
}