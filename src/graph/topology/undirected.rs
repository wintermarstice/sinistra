use crate::graph::{EdgeHandle, Topology, VertexHandle};

pub struct Undirected<T>(T);

impl<T> Undirected<T> {
    pub const fn new(topology: T) -> Self {
        Self(topology)
    }
}

impl<T: Topology> Topology for Undirected<T> {
    type Vertices<'a>
        = T::Vertices<'a>
    where
        Self: 'a;

    type Edges<'a>
        = T::Edges<'a>
    where
        Self: 'a;

    type OutNeighbors<'a>
        = std::iter::Chain<T::OutNeighbors<'a>, T::InNeighbors<'a>>
    where
        Self: 'a;

    type InNeighbors<'a>
        = std::iter::Chain<T::OutNeighbors<'a>, T::InNeighbors<'a>>
    where
        Self: 'a;

    type OutEdges<'a>
        = T::OutEdges<'a>
    where
        Self: 'a;

    type InEdges<'a>
        = T::InEdges<'a>
    where
        Self: 'a;

    type Adjacent<'a>
        = T::Adjacent<'a>
    where
        Self: 'a;

    fn edges(&self) -> Self::Edges<'_> {
        self.0.edges()
    }

    fn vertices(&self) -> Self::Vertices<'_> {
        self.0.vertices()
    }

    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        self.0.edge_endpoints(edge)
    }

    fn out_neighbors(&self, v: VertexHandle) -> Self::OutNeighbors<'_> {
        self.0.out_neighbors(v).chain(self.0.in_neighbors(v))
    }

    fn in_neighbors(&self, v: VertexHandle) -> Self::InNeighbors<'_> {
        self.0.out_neighbors(v).chain(self.0.in_neighbors(v))
    }

    fn out_edges(&self, v: VertexHandle) -> Self::OutEdges<'_> {
        self.0.out_edges(v)
    }

    fn in_edges(&self, v: VertexHandle) -> Self::InEdges<'_> {
        self.0.in_edges(v)
    }

    fn adjacent(&self, v: VertexHandle) -> Self::Adjacent<'_> {
        self.0.adjacent(v)
    }
}
