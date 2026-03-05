mod generational;
mod hash;

pub use generational::{GenerationalColorMap, GenerationalVisitMap};

use crate::graph::VertexHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Gray,
    Black,
}

pub trait ColorMap {
    fn color(&self, handle: VertexHandle) -> Color;
    fn set_color(&mut self, handle: VertexHandle, color: Color) -> Option<Color>;
}

pub trait VisitMap {
    fn visit(&mut self, vertex: VertexHandle) -> bool;
    fn is_visited(&self, vertex: VertexHandle) -> bool;
}

impl<T: ColorMap> VisitMap for T {
    fn visit(&mut self, vertex: VertexHandle) -> bool {
        match self.color(vertex) {
            Color::White => {
                self.set_color(vertex, Color::Gray);
                true
            }
            _ => false,
        }
    }

    fn is_visited(&self, vertex: VertexHandle) -> bool {
        matches!(self.color(vertex), Color::Gray | Color::Black)
    }
}
