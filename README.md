# gtk4cmark-rs

[![License](https://img.shields.io/github/license/Inparsian/gtk4cmark-rs)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/gtk4cmark)](https://crates.io/crates/gtk4cmark)
![Top language](https://img.shields.io/github/languages/top/Inparsian/gtk4cmark-rs)

A GTK4 widget for rendering CommonMark content.

- Written in Rust
- Native GTK4 widget
- Currently supports rendering of basic CommonMark elements
- Does not re-render unchanged content

## Example

```rust
let bx = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

// Create a MarkdownView and set some markdown content
let markdown_view = MarkdownView::new();
markdown_view.set_markdown("# Hello, GTK4Cmark!\nThis is a *CommonMark* example.");

// MarkdownView inherits from gtk4::Widget, so you can add it to containers
bx.append(&markdown_view);
```
