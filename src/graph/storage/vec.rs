use std::sync::atomic::{AtomicU32, Ordering};

use crate::graph::{Checked, EdgeHandle, Storage, VertexHandle};

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

    fn checked_vertex(&self, handle: VertexHandle) -> Option<Checked<VertexHandle>> {
        let checked = Checked::graph_and_generation(handle, self.discriminant)?;
        if checked.index() >= self.vertices.len() {
            return None;
        }

        Some(checked)
    }

    fn checked_edge(&self, handle: EdgeHandle) -> Option<Checked<EdgeHandle>> {
        let checked = Checked::graph_and_generation(handle, self.discriminant)?;
        if checked.index() >= self.edges.len() {
            return None;
        }

        Some(checked)
    }
}

impl<V, E> Storage for VecStorage<V, E> {
    type Vertex = V;
    type Edge = E;

    fn vertex(&self, handle: VertexHandle) -> Option<&Self::Vertex> {
        let checked = self.checked_vertex(handle)?;
        self.vertices.get(checked.index())?.as_ref()
    }

    fn edge(&self, handle: EdgeHandle) -> Option<&Self::Edge> {
        let checked = self.checked_edge(handle)?;
        self.edges.get(checked.index())?.as_ref()
    }

    fn vertex_mut(&mut self, handle: VertexHandle) -> Option<&mut Self::Vertex> {
        let checked = self.checked_vertex(handle)?;
        self.vertices.get_mut(checked.index())?.as_mut()
    }

    fn edge_mut(&mut self, handle: EdgeHandle) -> Option<&mut Self::Edge> {
        let checked = self.checked_edge(handle)?;
        self.edges.get_mut(checked.index())?.as_mut()
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
        let checked = self.checked_vertex(handle)?;
        self.vertices[checked.index()].replace(vertex)
    }

    fn set_edge(&mut self, handle: EdgeHandle, edge: Self::Edge) -> Option<Self::Edge> {
        let checked = self.checked_edge(handle)?;
        self.edges[checked.index()].replace(edge)
    }

    fn remove_vertex(&mut self, handle: VertexHandle) -> Option<Self::Vertex> {
        let checked = self.checked_vertex(handle)?;
        self.vertices[checked.index()].take()
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<Self::Edge> {
        let checked = self.checked_edge(handle)?;
        self.edges[checked.index()].take()
    }
}
