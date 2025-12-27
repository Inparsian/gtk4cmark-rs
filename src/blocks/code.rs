use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use gtk4::prelude::*;
use sourceview5::{View, Buffer, LanguageManager, StyleSchemeManager, BackgroundPatternType};
use sourceview5::prelude::*;
use markdown::mdast::Node;

use super::{BlockWidget, BlockWidgetFactory};

#[derive(Debug, Clone)]
pub struct CodeBlock {
    root: gtk4::Box,
    source_view: View,
    lang_label: Option<gtk4::Label>,
    lang: Option<String>,
    line_cache: Rc<RefCell<HashMap<usize, (String, String)>>>,
}

impl BlockWidget for CodeBlock {
    fn root(&self) -> &gtk4::Widget {
        self.root.upcast_ref()
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
            root: self.root.clone(),
            source_view: self.source_view.clone(),
            lang_label: self.lang_label.clone(),
            lang: self.lang.clone(),
            line_cache: Rc::new(RefCell::new(HashMap::new())),
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

        let window = gtk4::ScrolledWindow::builder()
            .css_classes(["cmark-codeblock-window"])
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .propagate_natural_height(true)
            .max_content_height(250)
            .vexpand_set(true)
            .child(&source_view)
            .build();

        let header_box = gtk4::Box::builder()
            .css_classes(["cmark-codeblock-header"])
            .orientation(gtk4::Orientation::Horizontal)
            .halign(gtk4::Align::Fill)
            .valign(gtk4::Align::Start)
            .build();

        let lang_label = gtk4::Label::builder()
            .css_classes(["cmark-codeblock-lang"])
            .justify(gtk4::Justification::Left)
            .halign(gtk4::Align::Start)
            .valign(gtk4::Align::Start)
            .wrap(true)
            .wrap_mode(gtk4::pango::WrapMode::WordChar)
            .selectable(false)
            .label("plaintext")
            .build();

        header_box.append(&lang_label);

        let root = gtk4::Box::builder()
            .css_classes(["cmark-codeblock"])
            .orientation(gtk4::Orientation::Vertical)
            .halign(gtk4::Align::Fill)
            .valign(gtk4::Align::Start)           
            .overflow(gtk4::Overflow::Hidden)
            .build();

        root.append(&header_box);
        root.append(&window);

        Self {
            root,
            source_view,
            lang: None,
            lang_label: Some(lang_label),
            line_cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl CodeBlock {
    fn set_lang(&mut self, lang: Option<&String>) {
        let old_lang = self.lang.clone();
        if old_lang == lang.cloned() {
            return;
        }
        
        self.lang = lang.cloned();
        if let Some(lang_label) = &self.lang_label {
            lang_label.set_label(
                lang.as_ref()
                    .map_or("plaintext", |l| l.as_str()),
            );

            let buffer = self.source_view.buffer();
            let buffer = buffer
                .downcast_ref::<Buffer>()
                .expect("Buffer is not a SourceView5 Buffer");

            if let Some(language) = LanguageManager::new().language(lang.unwrap_or(&"plaintext".to_owned())) {
                buffer.set_language(Some(&language));
            } else {
                buffer.set_language(None);
            }
        }
        self.line_cache.borrow_mut().clear();
    }
    
    pub fn set_markup(&self, code: &str) {
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