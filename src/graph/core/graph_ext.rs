use crate::graph::{
    EdgeHandle, EdgeTopology, EdgeTopologyMut, EndpointTopology, Graph, GraphMut, NeighborTopology,
    Storage, StorageMut, VertexHandle, VertexSetMut,
};

type Vertex<S> = <S as Storage>::Vertex;
type Edge<S> = <S as Storage>::Edge;

pub trait GraphNeighborsExt: Graph
where
    Self::Topology: NeighborTopology,
{
    fn out_neighbors(
        &self,
        handle: VertexHandle,
    ) -> <Self::Topology as NeighborTopology>::OutNeighbors<'_> {
        self.topology().out_neighbors(handle)
    }

    fn in_neighbors(
        &self,
        handle: VertexHandle,
    ) -> <Self::Topology as NeighborTopology>::InNeighbors<'_> {
        self.topology().in_neighbors(handle)
    }
}

pub trait GraphEdgesExt: Graph
where
    Self::Topology: EdgeTopology,
{
    fn edges(&self) -> <Self::Topology as EdgeTopology>::Edges<'_> {
        self.topology().edges()
    }

    fn out_edges(&self, handle: VertexHandle) -> <Self::Topology as EdgeTopology>::OutEdges<'_> {
        self.topology().out_edges(handle)
    }

    fn in_edges(&self, handle: VertexHandle) -> <Self::Topology as EdgeTopology>::InEdges<'_> {
        self.topology().in_edges(handle)
    }
}

pub trait GraphEndpointsExt: Graph
where
    Self::Topology: EndpointTopology,
{
    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        self.topology().edge_endpoints(edge)
    }
}

pub trait GraphVertexSetMutExt: GraphMut
where
    Self::TopologyMut: VertexSetMut,
{
    fn add_vertex(&mut self, vertex: Vertex<Self::Storage>) -> VertexHandle {
        let handle = self.storage_mut().add_vertex(vertex);
        self.topology_mut().add_vertex(handle);
        handle
    }

    fn remove_vertex(&mut self, handle: VertexHandle) -> Option<Vertex<Self::Storage>> {
        if self.topology_mut().remove_vertex(handle) {
            self.storage_mut().remove_vertex(handle)
        } else {
            None
        }
    }
}

pub trait GraphEdgesMutExt: GraphMut
where
    Self::TopologyMut: EdgeTopologyMut,
{
    fn add_edge(
        &mut self,
        edge: Edge<Self::Storage>,
        source: VertexHandle,
        target: VertexHandle,
    ) -> EdgeHandle {
        let handle = self.storage_mut().add_edge(edge);
        self.topology_mut().add_edge(handle, source, target);
        handle
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<Edge<Self::Storage>> {
        if self.topology_mut().remove_edge(handle) {
            self.storage_mut().remove_edge(handle)
        } else {
            None
        }
    }
}

impl<G> GraphNeighborsExt for G
where
    G: Graph,
    G::Topology: NeighborTopology,
{
}

impl<G> GraphEdgesExt for G
where
    G: Graph,
    G::Topology: EdgeTopology,
{
}

impl<G> GraphEndpointsExt for G
where
    G: Graph,
    G::Topology: EndpointTopology,
{
}

impl<G> GraphVertexSetMutExt for G
where
    G: GraphMut,
    G::TopologyMut: VertexSetMut,
{
}

impl<G> GraphEdgesMutExt for G
where
    G: GraphMut,
    G::TopologyMut: EdgeTopologyMut,
{
}
