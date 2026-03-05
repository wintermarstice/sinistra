use std::sync::atomic::{AtomicU32, Ordering};

use crate::graph::{EdgeHandle, Storage, VertexHandle};

static DISCRIMINANT_COUNTER: AtomicU32 = AtomicU32::new(10_000);

#[derive(Debug, Default, Clone)]
pub struct VecStorage<V, E> {
    discriminant: u32,
    vertices: Vec<Option<V>>,
    edges: Vec<Option<E>>,
}

impl<V, E> VecStorage<V, E> {
    pub fn new() -> Self {
        Self {
            discriminant: DISCRIMINANT_COUNTER.fetch_add(1, Ordering::Relaxed),
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn valid_vertex_handle(&self, handle: VertexHandle) -> Option<usize> {
        if handle.graph() != self.discriminant {
            return None;
        }

        let index = handle.index();
        if index >= self.vertices.len() {
            return None;
        }

        Some(index)
    }

    fn valid_edge_handle(&self, handle: EdgeHandle) -> Option<usize> {
        if handle.graph() != self.discriminant {
            return None;
        }

        let index = handle.index();
        if index >= self.edges.len() {
            return None;
        }

        Some(index)
    }
}

impl<V, E> Storage for VecStorage<V, E> {
    type Vertex = V;
    type Edge = E;

    fn vertex(&self, handle: VertexHandle) -> Option<&Self::Vertex> {
        let index = self.valid_vertex_handle(handle)?;
        self.vertices.get(index)?.as_ref()
    }

    fn edge(&self, handle: EdgeHandle) -> Option<&Self::Edge> {
        let index = self.valid_edge_handle(handle)?;
        self.edges.get(index)?.as_ref()
    }

    fn vertex_mut(&mut self, handle: VertexHandle) -> Option<&mut Self::Vertex> {
        let index = self.valid_vertex_handle(handle)?;
        self.vertices.get_mut(index)?.as_mut()
    }

    fn edge_mut(&mut self, handle: EdgeHandle) -> Option<&mut Self::Edge> {
        let index = self.valid_edge_handle(handle)?;
        self.edges.get_mut(index)?.as_mut()
    }

    fn add_vertex(&mut self, vertex: Self::Vertex) -> VertexHandle {
        let handle = VertexHandle::new(self.vertices.len() as u64, 1, self.discriminant);
        self.vertices.push(Some(vertex));
        handle
    }

    fn add_edge(&mut self, edge: Self::Edge) -> EdgeHandle {
        let handle = EdgeHandle::new(self.edges.len() as u64, 1, self.discriminant);
        self.edges.push(Some(edge));
        handle
    }

    fn set_vertex(&mut self, handle: VertexHandle, vertex: Self::Vertex) -> Option<Self::Vertex> {
        assert_eq!(
            handle.graph(),
            self.discriminant,
            "vertex handle graph mismatch"
        );

        let index = handle.index();
        if index >= self.vertices.len() {
            return None;
        }

        self.vertices[index].replace(vertex)
    }

    fn set_edge(&mut self, handle: EdgeHandle, edge: Self::Edge) -> Option<Self::Edge> {
        assert_eq!(
            handle.graph(),
            self.discriminant,
            "edge handle graph mismatch"
        );

        let index = handle.index();
        if index >= self.edges.len() {
            return None;
        }

        self.edges[index].replace(edge)
    }

    fn remove_vertex(&mut self, handle: VertexHandle) -> Option<Self::Vertex> {
        assert_eq!(
            handle.graph(),
            self.discriminant,
            "vertex handle graph mismatch"
        );

        let index = handle.index();
        if index >= self.vertices.len() {
            return None;
        }

        self.vertices[index].take()
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<Self::Edge> {
        assert_eq!(
            handle.graph(),
            self.discriminant,
            "edge handle graph mismatch"
        );

        let index = handle.index();
        if index >= self.edges.len() {
            return None;
        }

        self.edges[index].take()
    }
}
