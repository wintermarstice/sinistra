use std::collections::{HashMap, HashSet};

use crate::graph::{Color, ColorMap, VertexHandle, VisitMap};

impl VisitMap for HashSet<VertexHandle> {
    fn visit(&mut self, vertex: VertexHandle) -> bool {
        self.insert(vertex)
    }

    fn is_visited(&self, vertex: VertexHandle) -> bool {
        self.contains(&vertex)
    }
}

impl ColorMap for HashMap<VertexHandle, Color> {
    fn color(&self, handle: VertexHandle) -> Color {
        self.get(&handle).copied().unwrap_or(Color::White)
    }

    fn set_color(&mut self, handle: VertexHandle, color: Color) -> Option<Color> {
        self.insert(handle, color)
    }
}
