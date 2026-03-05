mod hash_map;
mod vec;

pub use hash_map::HashMapStorage;
pub use vec::VecStorage;

use crate::graph::{EdgeHandle, VertexHandle};

pub trait Storage {
    type Vertex;
    type Edge;

    fn vertex(&self, handle: VertexHandle) -> Option<&Self::Vertex>;
    fn edge(&self, handle: EdgeHandle) -> Option<&Self::Edge>;

    fn vertex_mut(&mut self, handle: VertexHandle) -> Option<&mut Self::Vertex>;
    fn edge_mut(&mut self, handle: EdgeHandle) -> Option<&mut Self::Edge>;
    fn add_vertex(&mut self, vertex: Self::Vertex) -> VertexHandle;
    fn add_edge(&mut self, edge: Self::Edge) -> EdgeHandle;
    fn set_vertex(&mut self, handle: VertexHandle, vertex: Self::Vertex) -> Option<Self::Vertex>;
    fn set_edge(&mut self, handle: EdgeHandle, edge: Self::Edge) -> Option<Self::Edge>;
    fn remove_vertex(&mut self, handle: VertexHandle) -> Option<Self::Vertex>;
    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<Self::Edge>;
}
