/*!
Tree View Component — Design System
Expandable/collapsible tree for Outline and Bookmark panels.
*/

use crate::ui::theme::*;
use eframe::egui;

/// A single node in the tree.
pub struct TreeNode {
    pub label: String,
    pub icon: Option<String>,
    pub children: Vec<TreeNode>,
    /// Arbitrary payload — e.g. page number for outline entries.
    pub data: Option<usize>,
}

impl TreeNode {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            children: Vec::new(),
            data: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn data(mut self, value: usize) -> Self {
        self.data = Some(value);
        self
    }

    pub fn child(mut self, node: TreeNode) -> Self {
        self.children.push(node);
        self
    }
}

/// Renders a tree from a list of root nodes.
/// Returns the `data` value of the clicked leaf, if any.
pub fn show_tree(ui: &mut egui::Ui, roots: &[TreeNode]) -> Option<usize> {
    let mut clicked = None;
    for node in roots {
        if let Some(val) = show_tree_node(ui, node, 0) {
            clicked = Some(val);
        }
    }
    clicked
}

fn show_tree_node(ui: &mut egui::Ui, node: &TreeNode, depth: usize) -> Option<usize> {
    let mut clicked = None;
    let indent = SP_LG * depth as f32;

    if node.children.is_empty() {
        // Leaf node
        ui.horizontal(|ui| {
            ui.add_space(indent + SP_SM);
            if let Some(icon) = &node.icon {
                ui.label(
                    egui::RichText::new(icon)
                        .size(FONT_SIZE_CAPTION)
                        .color(FG_TERTIARY)
                );
            }
            let resp = ui.selectable_label(
                false,
                egui::RichText::new(&node.label)
                    .size(FONT_SIZE_BODY)
                    .color(FG_PRIMARY)
            );
            if resp.clicked() {
                clicked = node.data;
            }
        });
    } else {
        // Branch node
        let id = ui.make_persistent_id(&node.label);
        let header_text = if let Some(icon) = &node.icon {
            format!("{icon}  {}", node.label)
        } else {
            node.label.clone()
        };

        ui.horizontal(|ui| {
            ui.add_space(indent);
        });

        egui::CollapsingHeader::new(
            egui::RichText::new(&header_text)
                .size(FONT_SIZE_BODY)
                .color(FG_PRIMARY)
        )
        .id_salt(id)
        .default_open(depth == 0)
        .show(ui, |ui| {
            for child in &node.children {
                if let Some(val) = show_tree_node(ui, child, depth + 1) {
                    clicked = Some(val);
                }
            }
        });
    }

    clicked
}
