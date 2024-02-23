use std::collections::HashMap;
use crate::parse;
use crate::parse::Attribute;
use crate::template::elements::{Body, Element};

pub struct ComponentElement {
    source: parse::Element,
    body: Vec<Body>,
    attr: HashMap<String, Attribute>
}

impl Element for ComponentElement {
    fn name(&self) -> String {
        self.source.name.clone()
    }
}