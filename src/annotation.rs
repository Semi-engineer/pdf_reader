/*!
Annotation Manager
Handles PDF annotations (highlights, rectangles, text, etc.)
*/

use egui::{Color32, Pos2, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnnotationType {
    Highlight,
    Rectangle,
    Circle,
    Line,
    Arrow,
    Pen,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: u64,
    pub page: usize,
    pub rect: AnnotationRect,
    pub color: [u8; 4], // RGBA
    pub annotation_type: AnnotationType,
    pub points: Vec<AnnotationPoint>, // For pen/line/arrow
    pub text: Option<String>,         // For text annotations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl AnnotationRect {
    pub fn from_egui(rect: Rect) -> Self {
        Self {
            x: rect.min.x,
            y: rect.min.y,
            width: rect.width(),
            height: rect.height(),
        }
    }
    
    pub fn to_egui(&self) -> Rect {
        Rect::from_min_size(
            Pos2::new(self.x, self.y),
            egui::vec2(self.width, self.height),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationPoint {
    pub x: f32,
    pub y: f32,
}

impl AnnotationPoint {
    pub fn from_pos2(pos: Pos2) -> Self {
        Self { x: pos.x, y: pos.y }
    }
    
    pub fn to_pos2(&self) -> Pos2 {
        Pos2::new(self.x, self.y)
    }
}

pub struct AnnotationManager {
    annotations: Vec<Annotation>,
    next_id: u64,
}

impl AnnotationManager {
    pub fn new() -> Self {
        Self {
            annotations: Vec::new(),
            next_id: 1,
        }
    }
    
    /// Add new annotation
    pub fn add_annotation(
        &mut self,
        page: usize,
        rect: Rect,
        color: Color32,
        annotation_type: AnnotationType,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        self.annotations.push(Annotation {
            id,
            page,
            rect: AnnotationRect::from_egui(rect),
            color: color.to_array(),
            annotation_type,
            points: Vec::new(),
            text: None,
        });
        
        id
    }
    
    /// Add pen/line annotation with points
    pub fn add_stroke_annotation(
        &mut self,
        page: usize,
        points: Vec<Pos2>,
        color: Color32,
        annotation_type: AnnotationType,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        // Calculate bounding rect
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        
        for point in &points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }
        
        self.annotations.push(Annotation {
            id,
            page,
            rect: AnnotationRect {
                x: min_x,
                y: min_y,
                width: max_x - min_x,
                height: max_y - min_y,
            },
            color: color.to_array(),
            annotation_type,
            points: points.iter().map(|p| AnnotationPoint::from_pos2(*p)).collect(),
            text: None,
        });
        
        id
    }
    
    /// Get annotations for a specific page
    pub fn get_page_annotations(&self, page: usize) -> Vec<&Annotation> {
        self.annotations
            .iter()
            .filter(|a| a.page == page)
            .collect()
    }
    
    /// Get all annotations
    pub fn get_all_annotations(&self) -> &[Annotation] {
        &self.annotations
    }
    
    /// Remove annotation by ID
    pub fn remove_annotation(&mut self, id: u64) {
        self.annotations.retain(|a| a.id != id);
    }
    
    /// Clear all annotations
    pub fn clear(&mut self) {
        self.annotations.clear();
    }
    
    /// Clear annotations for a specific page
    pub fn clear_page(&mut self, page: usize) {
        self.annotations.retain(|a| a.page != page);
    }
}

impl Default for AnnotationManager {
    fn default() -> Self {
        Self::new()
    }
}
