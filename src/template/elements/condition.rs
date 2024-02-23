use std::collections::HashMap;
use crate::error::*;
use crate::parse;
use crate::parse::Attribute;
use crate::template::elements::Body;
use crate::template::elements::Element;

pub struct ConditionElement {
    source: parse::Element,
    body: Vec<Body>,
    attr: HashMap<String, Attribute>
}

impl Element for ConditionElement {
    fn name(&self) -> String {
        self.source.name.clone()
    }

    fn from_element(element: &crate::parse::Element) -> Result<Self> {
        Ok(Self {
            attr: element
                .attributes
                .iter()
                .map(|attr| (attr.name.clone(), attr))
                .collect(),
            body: element.body.iter().map(|i| match i {
                parse::Body::Element(el) => Body::Element(Element::from_element(el)?),
                parse::Body::Literal(lit) => Body::Literal(lit.clone()),
                parse::Body::Expression(expr) => Body::Script(expr.clone()),
            }).collect(),
            source: element.clone(),
        })
    }

    fn render(&self, depth: u64) -> String {
        String::new()
    }
}