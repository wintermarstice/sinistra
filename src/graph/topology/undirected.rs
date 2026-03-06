use crate::graph::{
    EdgeHandle, EdgeTopology, EdgeTopologyMut, EndpointTopology, NeighborTopology, Topology,
    TopologyMut, VertexHandle, VertexSet, VertexSetMut,
};

pub struct Undirected<T>(T);

impl<T> Undirected<T> {
    pub const fn new(topology: T) -> Self {
        Self(topology)
    }
}

impl<T: VertexSet> VertexSet for Undirected<T> {
    type Vertices<'a>
        = T::Vertices<'a>
    where
        Self: 'a;

    fn vertices(&self) -> Self::Vertices<'_> {
        self.0.vertices()
    }
}

impl<T: NeighborTopology> NeighborTopology for Undirected<T> {
    type OutNeighbors<'a>
        = std::iter::Chain<T::OutNeighbors<'a>, T::InNeighbors<'a>>
    where
        Self: 'a;

    type InNeighbors<'a>
        = std::iter::Chain<T::OutNeighbors<'a>, T::InNeighbors<'a>>
    where
        Self: 'a;

    fn out_neighbors(&self, v: VertexHandle) -> Self::OutNeighbors<'_> {
        // TODO: Deduplicate the neighbors
        self.0.out_neighbors(v).chain(self.0.in_neighbors(v))
    }

    fn in_neighbors(&self, v: VertexHandle) -> Self::InNeighbors<'_> {
        // TODO: Deduplicate the neighbors
        self.0.out_neighbors(v).chain(self.0.in_neighbors(v))
    }
}

impl<T: EdgeTopology> EdgeTopology for Undirected<T> {
    type Edges<'a>
        = T::Edges<'a>
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

    fn edges(&self) -> Self::Edges<'_> {
        self.0.edges()
    }

    fn out_edges(&self, v: VertexHandle) -> Self::OutEdges<'_> {
        self.0.out_edges(v)
    }

    fn in_edges(&self, v: VertexHandle) -> Self::InEdges<'_> {
        self.0.in_edges(v)
    }
}

impl<T: EndpointTopology> EndpointTopology for Undirected<T> {
    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        self.0.edge_endpoints(edge)
    }
}

impl<T: Topology> Topology for Undirected<T> {
    type Adjacent<'a>
        = T::Adjacent<'a>
    where
        Self: 'a;

    fn adjacent(&self, v: VertexHandle) -> Self::Adjacent<'_> {
        self.0.adjacent(v)
    }
}

impl<T: VertexSetMut> VertexSetMut for Undirected<T> {
    fn add_vertex(&mut self, handle: VertexHandle) -> bool {
        self.0.add_vertex(handle)
    }

    fn remove_vertex(&mut self, handle: VertexHandle) -> bool {
        self.0.remove_vertex(handle)
    }
}

impl<T: EdgeTopologyMut> EdgeTopologyMut for Undirected<T> {
    fn add_edge(&mut self, handle: EdgeHandle, source: VertexHandle, target: VertexHandle) -> bool {
        self.0.add_edge(handle, source, target)
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> bool {
        self.0.remove_edge(handle)
    }
}

impl<T: TopologyMut> TopologyMut for Undirected<T> {}
