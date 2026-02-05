use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use futures_signals::signal::Mutable;
use gtk4::prelude::*;
use gtk4::TextBuffer;
use sourceview5::{View, Buffer, LanguageManager, StyleSchemeManager, BackgroundPatternType};
use sourceview5::prelude::*;
use markdown::mdast::Node;

use super::{BlockWidget, BlockWidgetFactory};

#[derive(Debug, Clone)]
pub struct CodeBlock {
    source_view: View,
    line_cache: Rc<RefCell<HashMap<usize, (String, String)>>>,
    /// The container widget for the code block.
    pub container: gtk4::Box,
    /// The `ScrolledWindow` containing the source view.
    pub root: gtk4::ScrolledWindow,
    /// The language of the code block.
    pub lang: Mutable<Option<String>>,
}

impl BlockWidget for CodeBlock {
    fn root(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }

    fn update(&mut self, node: &Node) {
        if let Node::Code(code) = node {
            self.set_lang(code.lang.as_ref());
            self.set_markup(&code.value);
        }
    }

    fn valid_node(&self, node: &Node) -> bool {
        matches!(node, Node::Code(_))
    }

    fn clone(&self) -> Box<dyn BlockWidget> {
        Box::new(Self {
            source_view: self.source_view.clone(),
            line_cache: Rc::new(RefCell::new(HashMap::new())),
            container: self.container.clone(),
            root: self.root.clone(),
            lang: self.lang.clone(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for CodeBlock {
    fn default() -> Self {
        let buffer = Buffer::new(None);
        buffer.set_highlight_syntax(true);

        if let Some(scheme) = StyleSchemeManager::new().scheme("classic-dark") {
            buffer.set_style_scheme(Some(&scheme));
        }

        let source_view = View::builder()
            .buffer(&buffer)
            .css_classes(["cmark-codeblock-sourceview"])
            .valign(gtk4::Align::Fill)
            .vexpand(true)
            .editable(false)
            .monospace(true)
            .tab_width(4)
            .show_line_numbers(true)
            .highlight_current_line(true)
            .background_pattern(BackgroundPatternType::None)
            .build();

        let root = gtk4::ScrolledWindow::builder()
            .css_classes(["cmark-codeblock-window"])
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .propagate_natural_height(true)
            .max_content_height(250)
            .vexpand_set(true)
            .child(&source_view)
            .build();

        let container = gtk4::Box::builder()
            .css_classes(["cmark-codeblock"])
            .orientation(gtk4::Orientation::Vertical)
            .halign(gtk4::Align::Fill)
            .valign(gtk4::Align::Start)           
            .overflow(gtk4::Overflow::Hidden)
            .build();

        container.append(&root);

        Self {
            source_view,
            line_cache: Rc::new(RefCell::new(HashMap::new())),
            container,
            root,
            lang: Mutable::new(None),
        }
    }
}

impl CodeBlock {
    /// Gets the `TextBuffer` for the underlying source view.
    pub fn buffer(&self) -> TextBuffer {
        self.source_view.buffer()
    }
    
    fn set_lang(&self, lang: Option<&String>) {
        let old_lang = self.lang.clone();
        if old_lang.get_cloned() == lang.cloned() {
            return;
        }
        
        self.lang.set(lang.cloned());
        let buffer = self.source_view.buffer();
        let buffer = buffer
            .downcast_ref::<Buffer>()
            .expect("Buffer is not a SourceView5 Buffer");

        if let Some(language) = LanguageManager::new().language(lang.unwrap_or(&"plaintext".to_owned())) {
            buffer.set_language(Some(&language));
        } else {
            buffer.set_language(None);
        }
        self.line_cache.borrow_mut().clear();
    }
    
    fn set_markup(&self, code: &str) {
        let buffer = self.source_view.buffer();
        let start = buffer.start_iter();
        let end = buffer.end_iter();
        let include_hidden_chars = true;
        if buffer.text(&start, &end, include_hidden_chars) != code {
            buffer.set_text(code);
        }
    }
}

pub struct CodeBlockFactory;
impl BlockWidgetFactory for CodeBlockFactory {
    fn create(&self) -> Box<dyn BlockWidget> {
        Box::new(CodeBlock::default())
    }

    fn matches(&self, node: &Node) -> bool {
        matches!(node, Node::Code(_))
    }
}