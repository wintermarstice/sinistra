use crate::graph::{EdgeHandle, Storage, Topology, VertexHandle, VertexSet};

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
