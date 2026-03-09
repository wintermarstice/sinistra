/// A handle to an edge in a graph.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct EdgeHandle {
    index: u64,
    generation: u32,
    graph: u32,
}

impl EdgeHandle {
    /// Creates a new edge handle.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the edge.
    /// * `generation` - The generation of the edge.
    /// * `graph` - The graph ID the edge belongs to.
    pub const fn new(index: u64, generation: u32, graph: u32) -> Self {
        Self {
            index,
            generation,
            graph,
        }
    }

    /// Returns the index of the edge.
    pub const fn index(&self) -> usize {
        self.index as usize
    }

    /// Returns the generation of the edge.
    pub const fn generation(&self) -> u32 {
        self.generation
    }

    /// Returns the graph ID the edge belongs to.
    pub const fn graph(&self) -> u32 {
        self.graph
    }
}

/// A handle to a vertex in a graph.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VertexHandle {
    index: u64,
    generation: u32,
    graph: u32,
}

impl VertexHandle {
    /// Creates a new vertex handle.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the vertex.
    /// * `generation` - The generation of the vertex.
    /// * `graph` - The graph ID the vertex belongs to.
    pub const fn new(index: u64, generation: u32, graph: u32) -> Self {
        Self {
            index,
            generation,
            graph,
        }
    }

    /// Returns the index of the vertex.
    pub const fn index(&self) -> usize {
        self.index as usize
    }

    /// Returns the generation of the vertex.
    pub const fn generation(&self) -> u32 {
        self.generation
    }

    /// Returns the graph ID the vertex belongs to.
    pub const fn graph(&self) -> u32 {
        self.graph
    }
}

/// A trait for handles to graph elements.
///
/// Handles are used to reference graph elements (vertices or edges) in a graph.
///
/// # Examples
///
/// ```
/// use sinistra::graph::{EdgeHandle, VertexHandle};
///
/// let vertex = VertexHandle::new(0, 0, 0);
/// let edge = EdgeHandle::new(0, 0, 0);
/// ```
pub trait Handle: Copy {
    /// Returns the index of the handle.
    fn index(&self) -> usize;
    /// Returns the generation of the handle.
    fn generation(&self) -> u32;
    /// Returns the graph ID the handle belongs to.
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
