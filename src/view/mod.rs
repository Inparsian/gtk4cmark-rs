mod imp;

use gtk4::glib::subclass::types::ObjectSubclassIsExt as _;
use gtk4::prelude::*;
use gtk4::glib::{self, Object};

use crate::ir::RenderMarker;
use crate::blocks::{BlockWidget, CodeBlock};

const MARKER_SPACING: i32 = 4;

glib::wrapper! {
    pub struct MarkdownView(ObjectSubclass<imp::MarkdownView>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Default for MarkdownView {
    fn default() -> Self {
        Object::builder().build()
    }
}

impl MarkdownView {
    /// Sets the function that is run before a new code block is appended.
    pub fn set_code_block_callback<F>(&self, callback: F)
    where
        F: Fn(&CodeBlock) + 'static,
    {
        let imp = self.imp();
        *imp.code_block_callback.borrow_mut() = Some(Box::new(callback));
    }
}

#[derive(Debug, Clone)]
struct MarkdownBlock {
    root: gtk4::Widget,
    block: Box<dyn BlockWidget>,
}

impl MarkdownBlock {
    fn new(block: Box<dyn BlockWidget>, marker: Option<&RenderMarker>) -> Self {
        let root = marker.map_or_else(|| block.root().clone(), |marker| {
            let indicator = match marker {
                RenderMarker::Bullet => "•",
                RenderMarker::Ordered(index) => &format!("{}.", index),
            };

            let marker_label = gtk4::Label::builder()
                .css_classes(["marker-label"])
                .valign(gtk4::Align::Start)
                .label(indicator)
                .build();

            let marker_box = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .spacing(MARKER_SPACING)
                .css_classes(["marker-box"])
                .build();

            marker_box.append(&marker_label);
            marker_box.append(block.root());
            marker_box.upcast()
        });

        Self {
            root,
            block,
        }
    }
}