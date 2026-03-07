use crate::graph::{EdgeHandle, Storage, StorageMut, VertexHandle, VertexSet, VertexSetMut};

type Vertex<S> = <S as Storage>::Vertex;
type Edge<S> = <S as Storage>::Edge;
type Vertices<'a, T> = <T as VertexSet>::Vertices<'a>;

pub trait Graph {
    type Storage: Storage;
    type Topology: VertexSet;

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
    type StorageMut: StorageMut<Vertex = Vertex<Self::Storage>, Edge = Edge<Self::Storage>>;
    type TopologyMut: VertexSetMut;

    fn storage_mut(&mut self) -> &mut Self::StorageMut;
    fn topology_mut(&mut self) -> &mut Self::TopologyMut;

    fn vertex_mut(&mut self, handle: VertexHandle) -> Option<&mut Vertex<Self::Storage>> {
        self.storage_mut().vertex_mut(handle)
    }

    fn edge_mut(&mut self, handle: EdgeHandle) -> Option<&mut Edge<Self::Storage>> {
        self.storage_mut().edge_mut(handle)
    }
}
