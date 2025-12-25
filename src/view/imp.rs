use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::LazyLock;
use gtk4::glib::{self, Properties};
use gtk4::subclass::prelude::*;
use gtk4::prelude::*;
use markdown::ParseOptions;

use crate::util::get_widget_children;
use super::MarkdownBlock;
use super::super::ir::{RenderBuffer, RenderBlock};
use super::super::blocks::{BlockWidgetFactory, text, thematicbreak, code, table};

const DEPTH_MULTIPLIER: i32 = 16;
const MARKER_SPACING: i32 = 4;

static FACTORIES: LazyLock<Vec<Box<dyn BlockWidgetFactory + Send + Sync>>> = LazyLock::new(|| vec![
    Box::new(text::TextBlockFactory),
    Box::new(code::CodeBlockFactory),
    Box::new(table::TableBlockFactory),
    Box::new(thematicbreak::ThematicBreakBlockFactory),
]);

#[derive(Default, Properties)]
#[properties(wrapper_type = super::MarkdownView)]
pub struct MarkdownView {
    pub(super) buffer: Rc<RefCell<RenderBuffer>>,
    pub(super) blocks: Rc<RefCell<HashMap<usize, MarkdownBlock>>>,

    #[property(get, set)]
    markdown: Rc<RefCell<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MarkdownView {
    const NAME: &'static str = "MarkdownView";
    type Type = super::MarkdownView;
    type ParentType = gtk4::Box;
}

#[glib::derived_properties]
impl ObjectImpl for MarkdownView {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_orientation(gtk4::Orientation::Vertical);
        self.obj().set_spacing(16);

        self.obj().connect_markdown_notify(|view| {
            let markdown = view.markdown();
            view.imp().render(&markdown);
        });
    }
}

impl WidgetImpl for MarkdownView {}

impl BoxImpl for MarkdownView {}

impl MarkdownView {
    fn children(&self) -> Vec<gtk4::Widget> {
        let mut children = Vec::new();
        let mut widget = self.obj().first_child();
        while let Some(child) = widget {
            widget = child.next_sibling();
            children.push(child);
        }
        children
    }

    fn render(
        &self,
        markdown: &str,
    ) {
        let mdast = match markdown::to_mdast(markdown, &ParseOptions::gfm()) {
            Ok(mdast) => mdast,
            Err(err) => {
                eprintln!("Failed to parse markdown: {}", err);
                return;
            }
        };

        self.buffer.borrow_mut().set(&mdast);

        // Only remove widgets where their index is above blocks.len and it makes
        // sense to leave in the root, rest of the widgets will be reused.
        let mut children = self.children();
        self.blocks.borrow_mut().retain(|i, block| {
            let buffer_blocks = &self.buffer.borrow().blocks;
            let valid_node = if let Some(buffer_block) = buffer_blocks.get(*i) {
                block.block.valid_node(&buffer_block.node)
            } else {
                false
            };

            if !valid_node && children.contains(&block.root) {
                self.obj().remove(&block.root);
                children.retain(|c| c != &block.root);
                false
            } else {
                true
            }
        });

        for (i, block) in self.buffer.borrow().blocks.iter().enumerate() {
            if let Some(mut md_block) = self.add_block(i, block, &children) {
                md_block.block.update(&block.node);
            } else {
                #[cfg(debug_assertions)]
                {
                    eprintln!("Failed to create or reuse block for node: {:?}", block.node);
                }
            }
        }
    }

    fn get_continuation_depth(
        i: usize,
        block: &RenderBlock,
        children: &[gtk4::Widget],
    ) -> i32 {
        if i == 0 {
            return 0;
        }

        if block.is_continuation
            && let Some(previous_box) = children.get(i - 1)
            && let Ok(bx) = previous_box.clone().downcast::<gtk4::Box>()
        {
            for child in get_widget_children(&bx) {
                if let Ok(label) = &child.clone().downcast::<gtk4::Label>() {
                    // the Label might not be rendered yet, so we can't use it's width allocation.
                    let text = label.text();
                    let layout = label.create_pango_layout(Some(&text));
                    let (width, _) = layout.pixel_size();

                    return width + MARKER_SPACING;
                }
            }
        }
        0
    }

    fn create_or_reuse_block(
        &self,
        i: usize,
        block: &RenderBlock,
    ) -> Option<(MarkdownBlock, bool)> {
        if let Some(text_block) = self.blocks.borrow_mut().get_mut(&i)
            && text_block.block.valid_node(&block.node)
        {
            text_block.block.update(&block.node);
            return Some((text_block.clone(), true));
        }

        FACTORIES.iter()
            .find(|f| f.matches(&block.node))
            .map_or_else(|| {
                eprintln!("No factory found for node: {:?}", block.node);
                None
            }, 
            |factory| {
                let block_widget = factory.create();
                let markdown_block = MarkdownBlock::new(block_widget, block.marker.as_ref());
                self.blocks.borrow_mut().insert(i, markdown_block.clone());
                Some((markdown_block, false))
            })
    }

    fn add_block(
        &self,
        i: usize,
        block: &RenderBlock,
        children: &[gtk4::Widget],
    ) -> Option<MarkdownBlock> {
        let (text_block, reused) = self.create_or_reuse_block(i, block)?;
        let continuation_depth = Self::get_continuation_depth(i, block, children);

        text_block.root.set_margin_start((block.depth as i32 * DEPTH_MULTIPLIER) + continuation_depth);

        if !reused {
            self.obj().append(&text_block.root);
        }

        Some(text_block)
    }
}