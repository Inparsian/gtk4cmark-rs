use std::{cell::RefCell, rc::Rc};
use markdown::mdast::{Node, Table};
use gtk4::prelude::*;

use super::{BlockWidget, BlockWidgetFactory};
use super::super::util;

#[derive(Debug, Clone)]
pub struct TableBlock {
    root: gtk4::Grid,
    rows: Rc<RefCell<Vec<Vec<gtk4::Label>>>>,
}

impl BlockWidget for TableBlock {
    fn root(&self) -> &gtk4::Widget {
        self.root.upcast_ref()
    }

    fn update(&mut self, node: &Node) {
        if let Node::Table(table) = node {
            self.set_table(table);
        }
    }

    fn valid_node(&self, node: &Node) -> bool {
        matches!(node, Node::Table(_))
    }

    fn clone(&self) -> Box<dyn BlockWidget> {
        Box::new(Self {
            root: self.root.clone(),
            rows: self.rows.clone(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for TableBlock {
    fn default() -> Self {
        let root = gtk4::Grid::builder()
            .row_spacing(0)
            .column_spacing(0)
            .css_classes(["cmark-table"])
            .build();

        Self {
            root,
            rows: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl TableBlock {
    pub fn set_table(&self, table: &Table) {
        let rows = &table.children;

        self.ensure_rows(
            rows.len(),
            rows.first().map_or(0, |r| r.children().unwrap().len()),
        );

        for (r, row) in rows.iter().enumerate() {
            for (c, cell) in row.children().unwrap().iter().enumerate() {
                let cell_label = &self.rows.borrow()[r][c];
                let mut cell_text = String::new();

                for child in cell.children().unwrap() {
                    cell_text.push_str(&util::inline_node_to_pango_markup(child));
                }

                // Only overwrite the label's text if it has changed.
                if cell_label.label() != cell_text {
                    cell_label.set_markup(&cell_text);
                }
            }
        }
    }

    fn ensure_rows(&self, row_count: usize, col_count: usize) {
        while self.rows.borrow().len() > row_count {
            if let Some(row) = self.rows.borrow_mut().pop() {
                for cell in row {
                    self.root.remove(&cell);
                }
            }
        }
    
        for r in 0..self.rows.borrow().len() {
            while self.rows.borrow()[r].len() > col_count {
                if let Some(cell) = self.rows.borrow_mut()[r].pop() {
                    self.root.remove(&cell);
                }
            }
        }
    
        while self.rows.borrow().len() < row_count {
            self.rows.borrow_mut().push(Vec::new());
        }
    
        for r in 0..row_count {
            while self.rows.borrow()[r].len() < col_count {
                let c = self.rows.borrow()[r].len();
            
                let label = gtk4::Label::builder()
                    .wrap(true)
                    .wrap_mode(gtk4::pango::WrapMode::WordChar)
                    .selectable(true)
                    .hexpand(true)
                    .halign(gtk4::Align::Fill)
                    .css_classes(["cmark-table-cell"])
                    .label("")
                    .build();
            
                self.root.attach(&label, c as i32, r as i32, 1, 1);
                self.rows.borrow_mut()[r].push(label);
            }
        }
    }
}

pub struct TableBlockFactory;
impl BlockWidgetFactory for TableBlockFactory {
    fn create(&self) -> Box<dyn BlockWidget> {
        Box::new(TableBlock::default())
    }

    fn matches(&self, node: &Node) -> bool {
        matches!(node, Node::Table(_))
    }
}