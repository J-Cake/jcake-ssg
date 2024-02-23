pub mod block;
pub mod component;
pub mod escape;
pub mod include;
pub mod template;
pub mod condition;

use rune::alloc::HashMap;
use crate::error::*;
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

pub trait Element {
    fn name(&self) -> String;

    fn from_element(element: &ParseElement) -> Result<Box<dyn Element>> {
        return Ok(Box::new(match element.name.as_str() {
            "block" => BlockElement::from_element(element)?,
            "template" => TemplateElement::from_element(element)?,
            "include" => IncludeElement::from_element(element)?,
            "component" => ComponentElement::from_element(element)?,
            "escape" => EscapeElement::from_element(element)?,
            "condition" => ConditionElement::from_element(element)?,
            _ => GenericElement::from_element(element)?,
        }));
    }

    fn render(&self, depth: u64) -> String;
}

pub struct GenericElement {
    pub(crate) source: parse::Element,
    pub(crate) body: Vec<Body>,
    pub(crate) attr: HashMap<String, Attribute>
}

impl Element for GenericElement {
    fn name(&self) -> String {
        self.source.name.clone()
    }

    fn from_element(element: &ParseElement) -> Result<Self> {
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