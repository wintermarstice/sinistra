use crate::graph::{Color, ColorMap, VertexHandle, VisitMap};

pub struct GenerationalColorMap {
    generation: u32,
    generations: Vec<u32>,
    colors: Vec<Color>,
}

impl ColorMap for GenerationalColorMap {
    fn color(&self, handle: VertexHandle) -> Color {
        let index = handle.index();

        if index >= self.generations.len() {
            return Color::White;
        }

        if self.generations[index] == self.generation {
            self.colors[index]
        } else {
            Color::White
        }
    }

    fn set_color(&mut self, handle: VertexHandle, color: Color) -> Option<Color> {
        let index = handle.index();

        if index >= self.generations.len() {
            self.ensure_capacity(index + 1);
        }

        self.generations[index] = self.generation;
        let previous = self.colors[index];
        self.colors[index] = color;
        Some(previous)
    }
}

impl GenerationalColorMap {
    pub fn new(capacity: usize) -> Self {
        Self {
            generation: 1,
            generations: vec![0; capacity],
            colors: vec![Color::White; capacity],
        }
    }

    pub fn reset(&mut self) {
        self.generation = self.generation.wrapping_add(1);

        if self.generation == 0 {
            self.generations.fill(0);
            self.colors.fill(Color::White);
            self.generation = 1;
        }
    }

    pub fn ensure_capacity(&mut self, capacity: usize) {
        if self.generations.len() < capacity {
            self.generations.resize(capacity, 0);
            self.colors.resize(capacity, Color::White);
        }
    }
}

pub struct GenerationalVisitMap {
    generation: u32,
    generations: Vec<u32>,
}

impl GenerationalVisitMap {
    pub fn new(capacity: usize) -> Self {
        Self {
            generation: 1,
            generations: vec![0; capacity],
        }
    }

    pub fn reset(&mut self) {
        self.generation = self.generation.wrapping_add(1);

        if self.generation == 0 {
            self.generations.fill(0);
            self.generation = 1;
        }
    }

    pub fn ensure_capacity(&mut self, capacity: usize) {
        if self.generations.len() < capacity {
            self.generations.resize(capacity, 0);
        }
    }
}

impl VisitMap for GenerationalVisitMap {
    fn visit(&mut self, vertex: VertexHandle) -> bool {
        let index = vertex.index() as usize;
        let generation = self.generation;

        if index >= self.generations.len() {
            self.ensure_capacity(index + 1);
        }

        if self.generations[index] == generation {
            false
        } else {
            self.generations[index] = generation;
            true
        }
    }

    fn is_visited(&self, vertex: VertexHandle) -> bool {
        let index = vertex.index() as usize;
        let generation = self.generation;

        if index >= self.generations.len() {
            return false;
        }

        self.generations[index] == generation
    }
}
