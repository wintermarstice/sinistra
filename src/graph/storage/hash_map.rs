use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::graph::{EdgeHandle, Storage, VertexHandle};

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
}

impl<V, E> Storage for HashMapStorage<V, E> {
    type Vertex = V;
    type Edge = E;

    fn vertex(&self, handle: VertexHandle) -> Option<&V> {
        if handle.graph() != self.discriminant {
            return None;
        }

        self.vertices.get(&handle)
    }

    fn edge(&self, handle: EdgeHandle) -> Option<&E> {
        if handle.graph() != self.discriminant {
            return None;
        }

        self.edges.get(&handle)
    }

    fn vertex_mut(&mut self, handle: VertexHandle) -> Option<&mut V> {
        if handle.graph() != self.discriminant {
            return None;
        }

        self.vertices.get_mut(&handle)
    }

    fn edge_mut(&mut self, handle: EdgeHandle) -> Option<&mut E> {
        if handle.graph() != self.discriminant {
            return None;
        }

        self.edges.get_mut(&handle)
    }

    fn set_vertex(&mut self, handle: VertexHandle, vertex: V) -> Option<V> {
        assert_eq!(
            handle.graph(),
            self.discriminant,
            "vertex handle graph mismatch"
        );
        self.vertices.insert(handle, vertex)
    }

    fn set_edge(&mut self, handle: EdgeHandle, edge: E) -> Option<E> {
        assert_eq!(
            handle.graph(),
            self.discriminant,
            "edge handle graph mismatch"
        );
        self.edges.insert(handle, edge)
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
        assert_eq!(
            handle.graph(),
            self.discriminant,
            "vertex handle graph mismatch"
        );
        self.vertices.remove(&handle)
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<E> {
        assert_eq!(
            handle.graph(),
            self.discriminant,
            "edge handle graph mismatch"
        );
        self.edges.remove(&handle)
    }
}
