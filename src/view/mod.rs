mod imp;

use gtk4::prelude::*;
use gtk4::glib::{self, Object};

use crate::{blocks::BlockWidget, ir};

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

const MARKER_SPACING: i32 = 4;

#[derive(Debug, Clone)]
pub struct MarkdownBlock {
    root: gtk4::Widget,
    block: Box<dyn BlockWidget>,
}

impl MarkdownBlock {
    fn new(
        block: Box<dyn BlockWidget>,
        marker: Option<&ir::RenderMarker>,
    ) -> Self {
        let root = marker.map_or_else(|| block.root().clone(), |marker| {
            let indicator = match marker {
                ir::RenderMarker::Bullet => "•",
                ir::RenderMarker::Ordered(index) => &format!("{}.", index),
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