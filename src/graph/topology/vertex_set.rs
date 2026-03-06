use crate::graph::VertexHandle;

pub trait VertexSet {
    type Vertices<'a>: Iterator<Item = VertexHandle>
    where
        Self: 'a;

    fn vertices(&self) -> Self::Vertices<'_>;
}

pub trait VertexSetMut: VertexSet {
    fn add_vertex(&mut self, handle: VertexHandle) -> bool;
    fn remove_vertex(&mut self, handle: VertexHandle) -> bool;
}
