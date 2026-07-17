/*!
Command System
Central command dispatcher for all user actions
*/

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    // Document
    OpenDocument,
    SaveDocument,
    ExportPDF,
    PrintDocument,
    CloseDocument,
    
    // Navigation
    NextPage,
    PreviousPage,
    FirstPage,
    LastPage,
    GoToPage,
    
    // View
    ZoomIn,
    ZoomOut,
    ZoomFit,
    ZoomWidth,
    Zoom100,
    RotateLeft,
    RotateRight,
    
    // Layout
    ToggleSidebarLeft,
    ToggleSidebarRight,
    ToggleTwoPageMode,
    ToggleFullscreen,
    
    // Search
    Search,
    SearchNext,
    SearchPrevious,
    ClearSearch,
    
    // Annotation
    AddHighlight,
    AddNote,
    AddDrawing,
    DeleteAnnotation,
    
    // Selection
    SelectAll,
    CopyText,
    
    // Workspace
    ToggleCommandPalette,
    ToggleTheme,
    ShowProperties,
    
    // System
    Quit,
}

impl Command {
    pub fn name(&self) -> &'static str {
        match self {
            Self::OpenDocument => "Open Document",
            Self::SaveDocument => "Save Document",
            Self::ExportPDF => "Export PDF",
            Self::PrintDocument => "Print Document",
            Self::CloseDocument => "Close Document",
            Self::NextPage => "Next Page",
            Self::PreviousPage => "Previous Page",
            Self::FirstPage => "First Page",
            Self::LastPage => "Last Page",
            Self::GoToPage => "Go to Page",
            Self::ZoomIn => "Zoom In",
            Self::ZoomOut => "Zoom Out",
            Self::ZoomFit => "Fit Page",
            Self::ZoomWidth => "Fit Width",
            Self::Zoom100 => "Actual Size",
            Self::RotateLeft => "Rotate Left",
            Self::RotateRight => "Rotate Right",
            Self::ToggleSidebarLeft => "Toggle Left Sidebar",
            Self::ToggleSidebarRight => "Toggle Right Sidebar",
            Self::ToggleTwoPageMode => "Toggle Two-Page Mode",
            Self::ToggleFullscreen => "Toggle Fullscreen",
            Self::Search => "Search",
            Self::SearchNext => "Next Search Result",
            Self::SearchPrevious => "Previous Search Result",
            Self::ClearSearch => "Clear Search",
            Self::AddHighlight => "Add Highlight",
            Self::AddNote => "Add Note",
            Self::AddDrawing => "Add Drawing",
            Self::DeleteAnnotation => "Delete Annotation",
            Self::SelectAll => "Select All",
            Self::CopyText => "Copy Text",
            Self::ToggleCommandPalette => "Command Palette",
            Self::ToggleTheme => "Toggle Theme",
            Self::ShowProperties => "Show Properties",
            Self::Quit => "Quit",
        }
    }
    
    pub fn default_shortcut(&self) -> Option<Shortcut> {
        Some(match self {
            Self::OpenDocument => Shortcut::new(true, false, false, 'O'),
            Self::SaveDocument => Shortcut::new(true, false, false, 'S'),
            Self::PrintDocument => Shortcut::new(true, false, false, 'P'),
            Self::CloseDocument => Shortcut::new(true, false, false, 'W'),
            Self::NextPage => Shortcut::key(egui::Key::Space),
            Self::PreviousPage => Shortcut::shift_key(egui::Key::Space),
            Self::ZoomIn => Shortcut::ctrl_char('+'),
            Self::ZoomOut => Shortcut::ctrl_char('-'),
            Self::Zoom100 => Shortcut::ctrl_char('0'),
            Self::RotateLeft => Shortcut::ctrl_char('L'),
            Self::RotateRight => Shortcut::ctrl_char('R'),
            Self::Search => Shortcut::new(true, false, false, 'F'),
            Self::SearchNext => Shortcut::key(egui::Key::F3),
            Self::SearchPrevious => Shortcut::shift_key(egui::Key::F3),
            Self::CopyText => Shortcut::new(true, false, false, 'C'),
            Self::ToggleCommandPalette => Shortcut::new(true, true, false, 'P'),
            Self::Quit => Shortcut::new(true, false, false, 'Q'),
            _ => return None,
        })
    }
    
    pub fn category(&self) -> CommandCategory {
        match self {
            Self::OpenDocument | Self::SaveDocument | Self::ExportPDF 
            | Self::PrintDocument | Self::CloseDocument => CommandCategory::Document,
            
            Self::NextPage | Self::PreviousPage | Self::FirstPage 
            | Self::LastPage | Self::GoToPage => CommandCategory::Navigation,
            
            Self::ZoomIn | Self::ZoomOut | Self::ZoomFit | Self::ZoomWidth 
            | Self::Zoom100 | Self::RotateLeft | Self::RotateRight 
            | Self::ToggleTwoPageMode => CommandCategory::View,
            
            Self::Search | Self::SearchNext | Self::SearchPrevious 
            | Self::ClearSearch => CommandCategory::Search,
            
            Self::AddHighlight | Self::AddNote | Self::AddDrawing 
            | Self::DeleteAnnotation => CommandCategory::Annotation,
            
            _ => CommandCategory::System,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandCategory {
    Document,
    Navigation,
    View,
    Search,
    Annotation,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub key: Option<egui::Key>,
    pub char: Option<char>,
}

impl Shortcut {
    pub fn new(ctrl: bool, shift: bool, alt: bool, c: char) -> Self {
        Self {
            ctrl,
            shift,
            alt,
            key: None,
            char: Some(c),
        }
    }
    
    pub fn key(key: egui::Key) -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
            key: Some(key),
            char: None,
        }
    }
    
    pub fn ctrl_key(key: egui::Key) -> Self {
        Self {
            ctrl: true,
            shift: false,
            alt: false,
            key: Some(key),
            char: None,
        }
    }
    
    pub fn shift_key(key: egui::Key) -> Self {
        Self {
            ctrl: false,
            shift: true,
            alt: false,
            key: Some(key),
            char: None,
        }
    }
    
    pub fn ctrl_char(c: char) -> Self {
        Self::new(true, false, false, c)
    }
    
    pub fn format(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl { parts.push("Ctrl"); }
        if self.shift { parts.push("Shift"); }
        if self.alt { parts.push("Alt"); }
        
        if let Some(key) = self.key {
            parts.push(match key {
                egui::Key::Space => "Space",
                egui::Key::F3 => "F3",
                _ => "?",
            });
        } else if let Some(c) = self.char {
            parts.push(match c {
                '+' => "+",
                '-' => "-",
                _ => {
                    let mut buf = [0u8; 4];
                    c.encode_utf8(&mut buf);
                    return parts.join("+") + "+" + &c.to_uppercase().to_string();
                }
            });
        }
        
        parts.join("+")
    }
    
    pub fn matches(&self, ctx: &egui::Context) -> bool {
        let input = ctx.input(|i| i.clone());
        
        let modifiers_match = input.modifiers.ctrl == self.ctrl
            && input.modifiers.shift == self.shift
            && input.modifiers.alt == self.alt;
        
        if !modifiers_match {
            return false;
        }
        
        if let Some(key) = self.key {
            return input.key_pressed(key);
        }
        
        if let Some(c) = self.char {
            for event in &input.events {
                if let egui::Event::Key {
                    key: _,
                    pressed: true,
                    ..
                } = event
                {
                    // Check character input
                    if input.events.iter().any(|e| {
                        if let egui::Event::Text(text) = e {
                            text.to_lowercase() == c.to_lowercase().to_string()
                        } else {
                            false
                        }
                    }) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}

pub struct CommandDispatcher {
    shortcuts: HashMap<Command, Shortcut>,
    pending: Vec<Command>,
}

impl CommandDispatcher {
    pub fn new() -> Self {
        let mut shortcuts = HashMap::new();
        
        // Register all default shortcuts
        for cmd in [
            Command::OpenDocument,
            Command::SaveDocument,
            Command::PrintDocument,
            Command::CloseDocument,
            Command::NextPage,
            Command::PreviousPage,
            Command::ZoomIn,
            Command::ZoomOut,
            Command::Zoom100,
            Command::RotateLeft,
            Command::RotateRight,
            Command::Search,
            Command::SearchNext,
            Command::SearchPrevious,
            Command::CopyText,
            Command::ToggleCommandPalette,
            Command::Quit,
        ] {
            if let Some(shortcut) = cmd.default_shortcut() {
                shortcuts.insert(cmd, shortcut);
            }
        }
        
        Self {
            shortcuts,
            pending: Vec::new(),
        }
    }
    
    pub fn check_shortcuts(&mut self, ctx: &egui::Context) {
        for (cmd, shortcut) in &self.shortcuts {
            if shortcut.matches(ctx) {
                self.pending.push(*cmd);
            }
        }
    }
    
    pub fn execute(&mut self, cmd: Command) {
        self.pending.push(cmd);
    }
    
    pub fn take_pending(&mut self) -> Vec<Command> {
        std::mem::take(&mut self.pending)
    }
    
    pub fn get_shortcut(&self, cmd: Command) -> Option<&Shortcut> {
        self.shortcuts.get(&cmd)
    }
}

impl Default for CommandDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
