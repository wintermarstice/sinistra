use crate::graph::{
    EdgeHandle, Storage, Topology, TopologyMut, Undirected, VertexHandle, VertexSet,
};

type Vertex<S> = <S as Storage>::Vertex;
type Edge<S> = <S as Storage>::Edge;
type Vertices<'a, T> = <T as VertexSet>::Vertices<'a>;

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

    fn vertices(&self) -> Vertices<'_, Self::Topology> {
        self.topology().vertices()
    }
}

pub trait GraphMut: Graph {
    fn storage_mut(&mut self) -> &mut Self::Storage;
    fn topology_mut(&mut self) -> &mut Self::Topology;

    fn vertex_mut(&mut self, handle: VertexHandle) -> Option<&mut Vertex<Self::Storage>> {
        self.storage_mut().vertex_mut(handle)
    }

    fn edge_mut(&mut self, handle: EdgeHandle) -> Option<&mut Edge<Self::Storage>> {
        self.storage_mut().edge_mut(handle)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BasicGraph<S, T> {
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

impl<S: Storage, T: TopologyMut + Topology> GraphMut for BasicGraph<S, T> {
    fn storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.storage
    }

    fn topology_mut(&mut self) -> &mut Self::Topology {
        &mut self.topology
    }
}
