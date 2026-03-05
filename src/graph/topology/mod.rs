mod hash_map;
mod undirected;

pub use hash_map::HashMapTopology;
pub use undirected::Undirected;

use crate::graph::{EdgeHandle, VertexHandle};

pub trait Topology {
    type Vertices<'a>: Iterator<Item = VertexHandle>
    where
        Self: 'a;

    type Edges<'a>: Iterator<Item = EdgeHandle>
    where
        Self: 'a;

    type OutNeighbors<'a>: Iterator<Item = VertexHandle>
    where
        Self: 'a;

    type InNeighbors<'a>: Iterator<Item = VertexHandle>
    where
        Self: 'a;

    type OutEdges<'a>: Iterator<Item = EdgeHandle>
    where
        Self: 'a;

    type InEdges<'a>: Iterator<Item = EdgeHandle>
    where
        Self: 'a;

    type Adjacent<'a>: Iterator<Item = (VertexHandle, EdgeHandle)>
    where
        Self: 'a;

    fn vertices(&self) -> Self::Vertices<'_>;
    fn edges(&self) -> Self::Edges<'_>;
    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)>;
    fn out_neighbors(&self, v: VertexHandle) -> Self::OutNeighbors<'_>;
    fn in_neighbors(&self, v: VertexHandle) -> Self::InNeighbors<'_>;
    fn out_edges(&self, v: VertexHandle) -> Self::OutEdges<'_>;
    fn in_edges(&self, v: VertexHandle) -> Self::InEdges<'_>;
    fn adjacent(&self, v: VertexHandle) -> Self::Adjacent<'_>;
}

pub trait TopologyMut: Topology {
    fn remove_vertex(&mut self, handle: VertexHandle) -> bool;
    fn remove_edge(&mut self, handle: EdgeHandle) -> bool;

    fn add_vertex(&mut self, handle: VertexHandle) -> bool;
    fn add_edge(&mut self, handle: EdgeHandle, source: VertexHandle, target: VertexHandle) -> bool;
}
