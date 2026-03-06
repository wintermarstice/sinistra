use crate::graph::{
    EdgeHandle, VertexHandle,
    topology::{NeighborTopology, VertexSetMut},
};

pub trait EdgeTopology: NeighborTopology {
    type Edges<'a>: Iterator<Item = EdgeHandle>
    where
        Self: 'a;

    type OutEdges<'a>: Iterator<Item = EdgeHandle>
    where
        Self: 'a;

    type InEdges<'a>: Iterator<Item = EdgeHandle>
    where
        Self: 'a;

    fn edges(&self) -> Self::Edges<'_>;
    fn out_edges(&self, v: VertexHandle) -> Self::OutEdges<'_>;
    fn in_edges(&self, v: VertexHandle) -> Self::InEdges<'_>;
}

pub trait EdgeTopologyMut: EdgeTopology + VertexSetMut {
    fn add_edge(&mut self, handle: EdgeHandle, source: VertexHandle, target: VertexHandle) -> bool;
    fn remove_edge(&mut self, handle: EdgeHandle) -> bool;
}
