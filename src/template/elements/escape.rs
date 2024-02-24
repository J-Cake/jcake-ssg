use std::collections::HashMap;
use crate::error::*;
use crate::parse;
use crate::parse::Attribute;
use crate::template::elements::{Body, Element};

pub struct EscapeElement {
    pub(super) source: parse::Element,
    pub(super) body: Vec<Body>,
    pub(super) attr: HashMap<String, Attribute>
}

impl Element for EscapeElement {
    fn name(&self) -> String {
        self.source.name.clone()
    }

    fn render(&self, depth: u64) -> String {
        String::new()
    }
}
