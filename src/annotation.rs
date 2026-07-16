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
    pub color: [u8; 4],
    pub annotation_type: AnnotationType,
    pub points: Vec<AnnotationPoint>,
    pub text: Option<String>,
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
        Rect::from_min_size(Pos2::new(self.x, self.y), egui::vec2(self.width, self.height))
    }

    /// Return the rect scaled to screen pixels for the given zoom percentage.
    pub fn to_screen(&self, zoom: f32, page_origin: egui::Pos2) -> Rect {
        let scale = zoom / 100.0;
        Rect::from_min_size(
            egui::pos2(
                page_origin.x + self.x * scale,
                page_origin.y + self.y * scale,
            ),
            egui::vec2(self.width * scale, self.height * scale),
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

    /// Reserve the next ID (increments the counter).
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Insert a pre-built annotation.
    pub fn push(&mut self, annotation: Annotation) {
        self.annotations.push(annotation);
    }

    /// Convenience: create and insert an annotation from screen-space rect.
    pub fn add_annotation(
        &mut self,
        page: usize,
        rect: Rect,
        color: Color32,
        annotation_type: AnnotationType,
    ) -> u64 {
        let id = self.next_id();
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

    /// Add a stroke (pen/line) annotation.
    pub fn add_stroke_annotation(
        &mut self,
        page: usize,
        points: Vec<Pos2>,
        color: Color32,
        annotation_type: AnnotationType,
    ) -> u64 {
        let id = self.next_id();
        let (min_x, min_y, max_x, max_y) = points.iter().fold(
            (f32::MAX, f32::MAX, f32::MIN, f32::MIN),
            |(mnx, mny, mxx, mxy), p| {
                (mnx.min(p.x), mny.min(p.y), mxx.max(p.x), mxy.max(p.y))
            },
        );
        self.annotations.push(Annotation {
            id,
            page,
            rect: AnnotationRect { x: min_x, y: min_y, width: max_x - min_x, height: max_y - min_y },
            color: color.to_array(),
            annotation_type,
            points: points.iter().map(|p| AnnotationPoint::from_pos2(*p)).collect(),
            text: None,
        });
        id
    }

    pub fn get_page_annotations(&self, page: usize) -> Vec<&Annotation> {
        self.annotations.iter().filter(|a| a.page == page).collect()
    }

    pub fn get_all_annotations(&self) -> &[Annotation] {
        &self.annotations
    }

    pub fn remove_annotation(&mut self, id: u64) {
        self.annotations.retain(|a| a.id != id);
    }

    pub fn clear(&mut self) {
        self.annotations.clear();
    }

    pub fn clear_page(&mut self, page: usize) {
        self.annotations.retain(|a| a.page != page);
    }
}

impl Default for AnnotationManager {
    fn default() -> Self {
        Self::new()
    }
}
