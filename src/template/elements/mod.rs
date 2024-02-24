pub mod block;
pub mod component;
pub mod escape;
pub mod include;
pub mod template;
pub mod condition;

use std::collections::HashMap;
use crate::parse;
use crate::parse::Attribute;
use crate::parse::Expression;
use crate::parse::Literal;
use crate::parse::Element as ParseElement;
use crate::template::elements::block::BlockElement;
use crate::template::elements::component::ComponentElement;
use crate::template::elements::condition::ConditionElement;
use crate::template::elements::escape::EscapeElement;
use crate::template::elements::include::IncludeElement;
use crate::template::elements::template::TemplateElement;

pub enum Body {
    Element(Box<dyn Element>),
    Literal(Literal),
    Script(Expression)
}

pub fn from_element(element: &ParseElement) -> Box<dyn Element> {
    let attr = element
        .attributes
        .iter()
        .map(|attr| (attr.name.clone(), attr.clone()))
        .collect();
    let body = element.body.iter().map(|i| match i {
        parse::Body::Element(el) => Body::Element(from_element(el)),
        parse::Body::Literal(lit) => Body::Literal(lit.clone()),
        parse::Body::Expression(expr) => Body::Script(expr.clone()),
    }).collect::<Vec<_>>();
    let source = element.clone();

    return match element.name.as_str() {
        "block" => Box::new(BlockElement { attr, body, source }),
        "template" => Box::new(TemplateElement { attr, body, source }),
        "include" => Box::new(IncludeElement { attr, body, source }),
        "component" => Box::new(ComponentElement { attr, body, source }),
        "escape" => Box::new(EscapeElement { attr, body, source }),
        "condition" => Box::new(ConditionElement { attr, body, source }),
        _ => Box::new(GenericElement { attr, body, source }),
    };
}

pub trait Element {
    fn name(&self) -> String;

    fn render(&self, depth: u64) -> String;
}

pub struct GenericElement {
    attr: HashMap<String, Attribute>,
    body: Vec<Body>,
    source: parse::Element,
}

impl Element for GenericElement {
    fn name(&self) -> String {
        self.source.name.clone()
    }

    fn render(&self, depth: u64) -> String {
        String::new()
    }
}