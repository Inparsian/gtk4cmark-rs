pub mod code;
pub mod text;
pub mod table;
pub mod thematicbreak;

use std::{any::Any, fmt::{Debug, Formatter, Result}};
use markdown::mdast::Node;

pub trait BlockWidget: Any {
    fn root(&self) -> &gtk4::Widget;
    fn update(&mut self, node: &Node);
    fn valid_node(&self, node: &Node) -> bool;

    fn clone(&self) -> Box<dyn BlockWidget>;
    fn as_any(&self) -> &dyn Any;
}

impl Debug for dyn BlockWidget {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "BlockWidget")
    }
}

impl Clone for Box<dyn BlockWidget> {
    fn clone(&self) -> Box<dyn BlockWidget> {
        self.as_ref().clone()
    }
}

impl dyn BlockWidget {
    pub fn downcast_ref<T: BlockWidget>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

pub trait BlockWidgetFactory {
    fn create(&self) -> Box<dyn BlockWidget>;
    fn matches(&self, node: &Node) -> bool;
}