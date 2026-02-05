#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod blocks;
mod view;
mod ir;
mod util;

pub use view::MarkdownView;

// Re-export dependencies for convenience
pub use futures_signals;