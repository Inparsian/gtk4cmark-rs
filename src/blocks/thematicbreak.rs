use gtk4::prelude::*;
use markdown::mdast::Node;

use super::{BlockWidget, BlockWidgetFactory};

#[derive(Debug, Clone)]
pub struct ThematicBreakBlock {
    root: gtk4::Separator,
}

impl BlockWidget for ThematicBreakBlock {
    fn root(&self) -> &gtk4::Widget {
        self.root.upcast_ref()
    }

    fn update(&mut self, _node: &Node) {}

    fn valid_node(&self, node: &Node) -> bool {
        matches!(node, Node::ThematicBreak(_))
    }

    fn clone(&self) -> Box<dyn BlockWidget> {
        Box::new(Self {
            root: self.root.clone(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for ThematicBreakBlock {
    fn default() -> Self {
        let root = gtk4::Separator::builder()
            .css_classes(["cmark-thematic-break"])
            .halign(gtk4::Align::Fill)
            .build();

        Self {
            root,
        }
    }
}

pub struct ThematicBreakBlockFactory;
impl BlockWidgetFactory for ThematicBreakBlockFactory {
    fn create(&self) -> Box<dyn BlockWidget> {
        Box::new(ThematicBreakBlock::default())
    }

    fn matches(&self, node: &Node) -> bool {
        matches!(node, Node::ThematicBreak(_))
    }
}