/*!
Toolbar Component
Horizontal toolbar with grouped buttons
*/

pub struct ToolbarGroup {
    items: Vec<ToolbarItem>,
}

pub enum ToolbarItem {
    Button {
        icon: String,
        tooltip: String,
        enabled: bool,
    },
    Toggle {
        icon: String,
        tooltip: String,
        active: bool,
    },
    Separator,
    Space,
}

impl ToolbarGroup {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    pub fn button(mut self, icon: impl Into<String>, tooltip: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            icon: icon.into(),
            tooltip: tooltip.into(),
            enabled: true,
        });
        self
    }
    
    pub fn toggle(mut self, icon: impl Into<String>, tooltip: impl Into<String>, active: bool) -> Self {
        self.items.push(ToolbarItem::Toggle {
            icon: icon.into(),
            tooltip: tooltip.into(),
            active,
        });
        self
    }
    
    pub fn separator(mut self) -> Self {
        self.items.push(ToolbarItem::Separator);
        self
    }
    
    pub fn space(mut self) -> Self {
        self.items.push(ToolbarItem::Space);
        self
    }
}

impl Default for ToolbarGroup {
    fn default() -> Self {
        Self::new()
    }
}
