pub mod block;
pub mod component;
pub mod escape;
pub mod include;
pub mod template;

use rune::alloc::HashMap;
use crate::parse;
use crate::parse::{Attribute, Expression, Literal};

pub enum Body {
    Element(Box<dyn Element>),
    Literal(Literal),
    Script(Expression)
}

pub trait Element {
    fn name(&self) -> String;
}

pub struct GenericElement {
    source: parse::Element,
    body: Vec<Body>,
    attr: HashMap<String, Attribute>
}

impl Element for GenericElement {
    fn name(&self) -> String {
        self.source.name.clone()
    }
}
