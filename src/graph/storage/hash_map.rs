use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::graph::{Checked, EdgeHandle, Storage, StorageMut, VertexHandle};

static DISCRIMINANT_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Default, Clone)]
pub struct HashMapStorage<V, E> {
    discriminant: u32,
    edges: HashMap<EdgeHandle, E>,
    vertices: HashMap<VertexHandle, V>,
}

impl<V, E> HashMapStorage<V, E> {
    pub fn new() -> Self {
        Self {
            discriminant: DISCRIMINANT_COUNTER.fetch_add(1, Ordering::Relaxed),
            edges: HashMap::new(),
            vertices: HashMap::new(),
        }
    }

    fn checked_vertex(&self, handle: VertexHandle) -> Option<Checked<VertexHandle>> {
        Checked::graph_and_generation(handle, self.discriminant)
    }

    fn checked_edge(&self, handle: EdgeHandle) -> Option<Checked<EdgeHandle>> {
        Checked::graph_and_generation(handle, self.discriminant)
    }
}

impl<V, E> Storage for HashMapStorage<V, E> {
    type Vertex = V;
    type Edge = E;

    fn vertex(&self, handle: VertexHandle) -> Option<&V> {
        let checked = self.checked_vertex(handle)?;
        self.vertices.get(&checked.into_inner())
    }

    fn edge(&self, handle: EdgeHandle) -> Option<&E> {
        let checked = self.checked_edge(handle)?;
        self.edges.get(&checked.into_inner())
    }
}

impl<V, E> StorageMut for HashMapStorage<V, E> {
    fn vertex_mut(&mut self, handle: VertexHandle) -> Option<&mut V> {
        let checked = self.checked_vertex(handle)?;
        self.vertices.get_mut(&checked.into_inner())
    }

    fn edge_mut(&mut self, handle: EdgeHandle) -> Option<&mut E> {
        let checked = self.checked_edge(handle)?;
        self.edges.get_mut(&checked.into_inner())
    }

    fn set_vertex(&mut self, handle: VertexHandle, vertex: V) -> Option<V> {
        let checked = self.checked_vertex(handle)?;
        self.vertices.insert(checked.into_inner(), vertex)
    }

    fn set_edge(&mut self, handle: EdgeHandle, edge: E) -> Option<E> {
        let checked = self.checked_edge(handle)?;
        self.edges.insert(checked.into_inner(), edge)
    }

    fn add_vertex(&mut self, vertex: V) -> VertexHandle {
        let index = VertexHandle::new(self.vertices.len() as u64, 1, self.discriminant);
        self.vertices.insert(index, vertex);
        index
    }

    fn add_edge(&mut self, edge: E) -> EdgeHandle {
        let handle = EdgeHandle::new(self.edges.len() as u64, 1, self.discriminant);
        self.edges.insert(handle, edge);
        handle
    }

    fn remove_vertex(&mut self, handle: VertexHandle) -> Option<V> {
        let checked = self.checked_vertex(handle)?;
        self.vertices.remove(&checked.into_inner())
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<E> {
        let checked = self.checked_edge(handle)?;
        self.edges.remove(&checked.into_inner())
    }
}
