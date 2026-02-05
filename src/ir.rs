use markdown::mdast::{Node, Paragraph, Text};

use super::util;

/// Types of lists while rendering.
#[derive(Debug, Clone)]
pub enum RenderListType {
    Bullet,
    Ordered,
}

/// Marker for list items while rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderMarker {
    Bullet,
    Ordered(u32),
}

/// A block of content to be rendered.
#[derive(Debug, Clone)]
pub struct RenderBlock {
    pub node: Node,
    pub depth: usize,
    pub marker: Option<RenderMarker>,
    pub is_continuation: bool,
}

/// Represents the scope of a list while rendering.
#[derive(Debug, Clone)]
struct RenderListScope {
    list_type: RenderListType,
    next_marker_index: u32,
}

/// Context while walking the AST for rendering.
#[derive(Default, Debug, Clone)]
struct RenderWalkContext {
    list_stack: Vec<RenderListScope>,
    paragraph_stack: Vec<Vec<Node>>,
}

impl RenderWalkContext {
    fn depth(&self) -> usize {
        // A top-level list is depth 0
        self.list_stack.len().saturating_sub(1)
    }

    fn current_list(&self) -> Option<&RenderListScope> {
        self.list_stack.last()
    }

    fn next_marker(&mut self) -> u32 {
        let scope = self.list_stack.last_mut()
            .expect("next_marker() called outside of list scope");
        
        let index = scope.next_marker_index;
        scope.next_marker_index += 1;
        index
    }
}

/// An intermediate representation of the parsed markdown for rendering.
#[derive(Default, Debug, Clone)]
pub struct RenderBuffer {
    pub blocks: Vec<RenderBlock>,
}

impl RenderBuffer {
    /// Sets the render buffer by walking the AST starting from the given node.
    pub fn set(&mut self, node: &Node) {
        self.blocks.clear();
        let mut walk_ctx = RenderWalkContext::default();
        self.walk(node, &mut walk_ctx);
        self.drain_paragraph_stack(&mut walk_ctx);
    }
    
    /// Drains the paragraph stack.
    fn drain_paragraph_stack(&mut self, ctx: &mut RenderWalkContext) {
        if !ctx.paragraph_stack.is_empty() {
            let len = ctx.paragraph_stack.len();
            let mut children = Vec::new();
            for (i, paragraph) in ctx.paragraph_stack.drain(..).enumerate() {
                children.extend(paragraph);
                
                if i < len - 1 {
                    children.push(Node::Text(Text {
                        value: "\n\n".to_owned(),
                        position: None,
                    }));
                }
            }
            
            let node = Node::Paragraph(Paragraph {
                children,
                position: None,
            });
            
            self.push_block(RenderBlock {
                node,
                depth: ctx.depth(),
                marker: None,
                is_continuation: false,
            });
        }
    }

    /// Recursively walks the AST and populates the render buffer.
    fn walk(&mut self, node: &Node, ctx: &mut RenderWalkContext) {
        // Merge adjacent paragraphs
        if let Node::Paragraph(paragraph) = node {
            ctx.paragraph_stack.push(paragraph.children.clone());
            return;
        }
        
        self.drain_paragraph_stack(ctx);
        
        match node {
            Node::List(list) => if let Some(children) = node.children() {
                let list_type = if list.ordered {
                    RenderListType::Ordered
                } else {
                    RenderListType::Bullet
                };

                ctx.list_stack.push(RenderListScope {
                    list_type,
                    next_marker_index: list.start.unwrap_or(1),
                });

                for child in children {
                    self.walk(child, ctx);
                }

                ctx.list_stack.pop();
            },

            Node::ListItem(_) => if let Some(current_list) = ctx.current_list() {
                let mut first_block = true;

                let marker = match &current_list.list_type {
                    RenderListType::Bullet => RenderMarker::Bullet,
                    RenderListType::Ordered => RenderMarker::Ordered(ctx.next_marker())
                };

                if let Some(children) = node.children() {
                    for child in children {
                        if util::is_block_node(child) {
                            let is_first = first_block;
                            
                            let marker = is_first.then(|| {
                                first_block = false;
                                marker.clone()
                            });

                            self.push_block(RenderBlock {
                                node: child.clone(),
                                depth: ctx.depth(),
                                marker,
                                is_continuation: !is_first,
                            });
                        } else {
                            self.walk(child, ctx);
                        }
                    }
                }
            },

            _ => if util::is_block_node(node) {
                self.push_block(RenderBlock {
                    node: node.clone(),
                    depth: ctx.depth(),
                    marker: None,
                    is_continuation: false,
                });
            } else if let Some(children) = node.children() {
                #[cfg(debug_assertions)]
                {
                    let variant = util::node_variant_name(node);
                    if variant != "Root" {
                        println!("unhandled node: {}", variant);
                    }
                }

                for child in children {
                    self.walk(child, ctx);
                }
            }
        }
    }

    /// Pushes a block to the render buffer.
    fn push_block(&mut self, block: RenderBlock) {
        self.blocks.push(block);
    }
}