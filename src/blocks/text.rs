use gtk4::prelude::*;
use markdown::mdast::Node;

use super::{BlockWidget, BlockWidgetFactory};
use super::super::util;

const HEADER_SIZES: [(usize, &str); 6] = [
    (1, "170%"),
    (2, "150%"),
    (3, "125%"),
    (4, "110%"),
    (5, "100%"),
    (6, "90%"),
];

#[derive(Debug, Clone)]
pub struct TextBlock {
    root: gtk4::Label,
}

impl BlockWidget for TextBlock {
    fn root(&self) -> &gtk4::Widget {
        self.root.upcast_ref()
    }
    
    fn update(&mut self, node: &Node) {
        match node {
            Node::Paragraph(paragraph) => self.set_from_paragraph(paragraph),
            Node::Heading(heading) => self.set_from_heading(heading),
            _ => {}
        }
    }

    fn valid_node(&self, node: &Node) -> bool {
        matches!(node, Node::Paragraph(_) | Node::Heading(_))
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

impl Default for TextBlock {
    fn default() -> Self {
        let root = gtk4::Label::builder()
            .css_classes(["cmark-label"])
            .justify(gtk4::Justification::Left)
            .hexpand(true)
            .xalign(0.0)
            .wrap(true)
            .wrap_mode(gtk4::pango::WrapMode::WordChar)
            .selectable(true)
            .label("")
            .build();

        Self {
            root,
        }
    }
}

impl TextBlock {
    pub fn set_from_paragraph(&self, paragraph: &markdown::mdast::Paragraph) {
        let mut buffer = String::new();
        for inline in &paragraph.children {
            buffer.push_str(&util::inline_node_to_pango_markup(inline));
        }

        // Only overwrite the label's text if it has changed.
        if self.root.label() != buffer {
            self.root.set_markup(&buffer);
        }
    }

    pub fn set_from_heading(&self, heading: &markdown::mdast::Heading) {
        let heading_size = HEADER_SIZES
            .get(heading.depth as usize - 1)
            .map_or("100%", |(_, size)| *size);

        let mut buffer = String::new();
        for inline in &heading.children {
            buffer.push_str(&util::inline_node_to_pango_markup(inline));
        }

        // Only overwrite the label's text if it has changed.
        if self.root.label() != buffer {
            self.root.set_markup(&format!("<span size=\"{}\">{}</span>", heading_size, buffer));
        }
    }
}

pub struct TextBlockFactory;
impl BlockWidgetFactory for TextBlockFactory {
    fn create(&self) -> Box<dyn BlockWidget> {
        Box::new(TextBlock::default())
    }

    fn matches(&self, node: &Node) -> bool {
        matches!(node, Node::Paragraph(_) | Node::Heading(_))
    }
}