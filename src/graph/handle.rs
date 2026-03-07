#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct EdgeHandle {
    index: u64,
    generation: u32,
    graph: u32,
}

impl EdgeHandle {
    pub(crate) const fn new(index: u64, generation: u32, graph: u32) -> Self {
        Self {
            index,
            generation,
            graph,
        }
    }

    pub const fn index(&self) -> usize {
        self.index as usize
    }

    pub const fn generation(&self) -> u32 {
        self.generation
    }

    pub const fn graph(&self) -> u32 {
        self.graph
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VertexHandle {
    index: u64,
    generation: u32,
    graph: u32,
}

impl VertexHandle {
    pub(crate) const fn new(index: u64, generation: u32, graph: u32) -> Self {
        Self {
            index,
            generation,
            graph,
        }
    }

    pub const fn index(&self) -> usize {
        self.index as usize
    }

    pub const fn generation(&self) -> u32 {
        self.generation
    }

    pub const fn graph(&self) -> u32 {
        self.graph
    }
}

pub trait Handle: Copy {
    fn index(&self) -> usize;
    fn generation(&self) -> u32;
    fn graph(&self) -> u32;
}

impl Handle for VertexHandle {
    fn index(&self) -> usize {
        self.index()
    }

    fn generation(&self) -> u32 {
        self.generation()
    }

    fn graph(&self) -> u32 {
        self.graph()
    }
}

impl Handle for EdgeHandle {
    fn index(&self) -> usize {
        self.index()
    }

    fn generation(&self) -> u32 {
        self.generation()
    }

    fn graph(&self) -> u32 {
        self.graph()
    }
}
