use crate::graph::{Graph, GraphMut, Storage, Topology, TopologyMut, Undirected};

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
