use crate::graph::{EdgeHandle, Storage, Topology, TopologyMut, Undirected, VertexHandle};

type Vertex<S> = <S as Storage>::Vertex;
type Edge<S> = <S as Storage>::Edge;

pub trait Graph {
    type Storage: Storage;
    type Topology: Topology;

    fn storage(&self) -> &Self::Storage;
    fn topology(&self) -> &Self::Topology;

    fn vertex(&self, handle: VertexHandle) -> Option<&Vertex<Self::Storage>> {
        self.storage().vertex(handle)
    }

    fn edge(&self, handle: EdgeHandle) -> Option<&Edge<Self::Storage>> {
        self.storage().edge(handle)
    }

    fn vertices(&self) -> <Self::Topology as Topology>::Vertices<'_> {
        self.topology().vertices()
    }

    fn edges(&self) -> <Self::Topology as Topology>::Edges<'_> {
        self.topology().edges()
    }

    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        self.topology().edge_endpoints(edge)
    }

    fn out_neighbors(
        &self,
        handle: VertexHandle,
    ) -> <Self::Topology as Topology>::OutNeighbors<'_> {
        self.topology().out_neighbors(handle)
    }

    fn in_neighbors(&self, handle: VertexHandle) -> <Self::Topology as Topology>::InNeighbors<'_> {
        self.topology().in_neighbors(handle)
    }

    fn out_edges(&self, handle: VertexHandle) -> <Self::Topology as Topology>::OutEdges<'_> {
        self.topology().out_edges(handle)
    }
}

pub trait GraphMut: Graph
where
    Self::Topology: TopologyMut,
{
    fn storage_mut(&mut self) -> &mut Self::Storage;
    fn topology_mut(&mut self) -> &mut Self::Topology;

    fn vertex_mut(&mut self, handle: VertexHandle) -> Option<&mut Vertex<Self::Storage>> {
        self.storage_mut().vertex_mut(handle)
    }

    fn edge_mut(&mut self, handle: EdgeHandle) -> Option<&mut Edge<Self::Storage>> {
        self.storage_mut().edge_mut(handle)
    }

    fn add_vertex(&mut self, vertex: Vertex<Self::Storage>) -> VertexHandle {
        let handle = self.storage_mut().add_vertex(vertex);
        self.topology_mut().add_vertex(handle);
        handle
    }

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

    fn remove_vertex(&mut self, handle: VertexHandle) -> Option<Vertex<Self::Storage>> {
        if self.topology_mut().remove_vertex(handle) {
            self.storage_mut().remove_vertex(handle)
        } else {
            None
        }
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<Edge<Self::Storage>> {
        if self.topology_mut().remove_edge(handle) {
            self.storage_mut().remove_edge(handle)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BasicGraph<S: Storage, T: Topology> {
    storage: S,
    topology: T,
}

impl<S: Storage, T: Topology> BasicGraph<S, T> {
    pub fn new(storage: S, topology: T) -> Self {
        Self { storage, topology }
    }

    pub fn undirected(storage: S, topology: T) -> BasicGraph<S, Undirected<T>> {
        BasicGraph {
            storage,
            topology: Undirected::new(topology),
        }
    }
}

impl<S: Storage, T: Topology> Graph for BasicGraph<S, T> {
    type Storage = S;
    type Topology = T;

    fn storage(&self) -> &Self::Storage {
        &self.storage
    }

    fn topology(&self) -> &Self::Topology {
        &self.topology
    }
}

impl<S: Storage, T: TopologyMut> GraphMut for BasicGraph<S, T> {
    fn storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.storage
    }

    fn topology_mut(&mut self) -> &mut Self::Topology {
        &mut self.topology
    }
}
