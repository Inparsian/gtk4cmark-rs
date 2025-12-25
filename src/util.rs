use markdown::mdast::Node;

/// Returns the enum variant name for a markdown AST node (e.g. `Paragraph`).
#[cfg(debug_assertions)]
pub fn node_variant_name(node: &Node) -> &'static str {
    match node {
        Node::Root(_) => "Root",
        Node::Paragraph(_) => "Paragraph",
        Node::Heading(_) => "Heading",
        Node::Blockquote(_) => "Blockquote",
        Node::List(_) => "List",
        Node::ListItem(_) => "ListItem",
        Node::Code(_) => "Code",
        Node::InlineCode(_) => "InlineCode",
        Node::ThematicBreak(_) => "ThematicBreak",
        Node::Break(_) => "Break",
        Node::Text(_) => "Text",
        Node::Emphasis(_) => "Emphasis",
        Node::Strong(_) => "Strong",
        Node::Delete(_) => "Delete",
        Node::Link(_) => "Link",
        Node::LinkReference(_) => "LinkReference",
        Node::Image(_) => "Image",
        Node::ImageReference(_) => "ImageReference",
        Node::Html(_) => "Html",
        Node::Yaml(_) => "Yaml",
        Node::Toml(_) => "Toml",
        Node::Definition(_) => "Definition",
        Node::FootnoteDefinition(_) => "FootnoteDefinition",
        Node::FootnoteReference(_) => "FootnoteReference",
        Node::Table(_) => "Table",
        Node::TableRow(_) => "TableRow",
        Node::TableCell(_) => "TableCell",
        Node::MdxjsEsm(_) => "MdxjsEsm",
        Node::MdxFlowExpression(_) => "MdxFlowExpression",
        Node::MdxJsxFlowElement(_) => "MdxJsxFlowElement",
        Node::MdxJsxTextElement(_) => "MdxJsxTextElement",
        Node::MdxTextExpression(_) => "MdxTextExpression",
        Node::Math(_) => "Math",
        Node::InlineMath(_) => "InlineMath",
    }
}

/// Returns true if the node is a renderable block-level node.
/// Structural nodes such as lists and blockquotes are intentionally excluded.
pub fn is_block_node(node: &Node) -> bool {
    matches!(
        node,
        Node::Paragraph(_)
           // | Node::Definition(_)
            | Node::Heading(_)
            | Node::Code(_)
            | Node::Table(_)
            | Node::ThematicBreak(_)
    )
}

/// Escapes special characters in a string for use in XML.
pub fn xml_escape(input: &str) -> String {
    input.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Gets an HTML tag
pub fn get_html_tag(input: &str) -> Option<String> {
    let input = input.trim_start();
    if !input.starts_with('<') {
        return None;
    }

    let end = input.find('>')?;
    let tag_content = &input[1..end];
    let tag_name = tag_content
        .split_whitespace()
        .next()?
        .trim_start_matches('/')
        .trim_end_matches('/');

    Some(tag_name.to_owned())
}

/// Converts an inline node into pango markup.
pub fn inline_node_to_pango_markup(node: &Node) -> String {
    match node {
        Node::Text(text) => xml_escape(&text.value),
        Node::Break(_) => "\n".to_owned(),
        Node::Emphasis(emphasis) => format!("<i>{}</i>", emphasis.children.iter().map(inline_node_to_pango_markup).collect::<String>()),
        Node::Strong(strong) => format!("<b>{}</b>", strong.children.iter().map(inline_node_to_pango_markup).collect::<String>()),
        Node::Link(link) => format!("<a href=\"{}\">{}</a>", xml_escape(&link.url), link.children.iter().map(inline_node_to_pango_markup).collect::<String>()),
        Node::Delete(delete) => format!("<s>{}</s>", delete.children.iter().map(inline_node_to_pango_markup).collect::<String>()),
        Node::InlineCode(code) => format!("<tt>{}</tt>", xml_escape(&code.value)),
        Node::Html(html) => if let Some(tag) = get_html_tag(&html.value) && tag.eq_ignore_ascii_case("br") {
            '\n'.to_string()
        } else {
            xml_escape(&html.value)
        },
        _ => String::new()
    }
}