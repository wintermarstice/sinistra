use crate::graph::{VertexHandle, topology::VertexSet};

pub trait NeighborTopology: VertexSet {
    type OutNeighbors<'a>: Iterator<Item = VertexHandle>
    where
        Self: 'a;

    type InNeighbors<'a>: Iterator<Item = VertexHandle>
    where
        Self: 'a;

    fn out_neighbors(&self, v: VertexHandle) -> Self::OutNeighbors<'_>;
    fn in_neighbors(&self, v: VertexHandle) -> Self::InNeighbors<'_>;
}
