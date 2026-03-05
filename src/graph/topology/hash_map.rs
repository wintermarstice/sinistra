use std::{
    collections::{HashMap, HashSet},
    iter::Copied,
};

use crate::graph::{EdgeHandle, Topology, TopologyMut, VertexHandle};

#[derive(Debug, Default, Clone)]
pub struct HashMapTopology {
    vertices: HashSet<VertexHandle>,
    edges: HashMap<EdgeHandle, (VertexHandle, VertexHandle)>,
    out_edges: HashMap<VertexHandle, Vec<EdgeHandle>>,
    in_edges: HashMap<VertexHandle, Vec<EdgeHandle>>,
}

impl HashMapTopology {
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashMap::new(),
            out_edges: HashMap::new(),
            in_edges: HashMap::new(),
        }
    }
}

impl Topology for HashMapTopology {
    type Vertices<'a>
        = Copied<std::collections::hash_set::Iter<'a, VertexHandle>>
    where
        Self: 'a;

    type Edges<'a>
        = Copied<std::collections::hash_map::Keys<'a, EdgeHandle, (VertexHandle, VertexHandle)>>
    where
        Self: 'a;

    type OutEdges<'a>
        = Copied<std::slice::Iter<'a, EdgeHandle>>
    where
        Self: 'a;

    type InEdges<'a>
        = Copied<std::slice::Iter<'a, EdgeHandle>>
    where
        Self: 'a;

    type OutNeighbors<'a>
        = OutNeighbors<'a>
    where
        Self: 'a;

    type InNeighbors<'a>
        = InNeighbors<'a>
    where
        Self: 'a;

    type Adjacent<'a>
        = Adjacent<'a>
    where
        Self: 'a;

    fn vertices(&self) -> Self::Vertices<'_> {
        self.vertices.iter().copied()
    }

    fn edges(&self) -> Self::Edges<'_> {
        self.edges.keys().copied()
    }

    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        self.edges.get(&edge).copied()
    }

    fn out_edges(&self, v: VertexHandle) -> Self::OutEdges<'_> {
        self.out_edges
            .get(&v)
            .map(|v| v.iter())
            .unwrap_or([].iter())
            .copied()
    }

    fn in_edges(&self, v: VertexHandle) -> Self::InEdges<'_> {
        self.in_edges
            .get(&v)
            .map(|v| v.iter())
            .unwrap_or([].iter())
            .copied()
    }

    fn out_neighbors(&self, v: VertexHandle) -> Self::OutNeighbors<'_> {
        let edges = self
            .out_edges
            .get(&v)
            .map(|v| v.iter())
            .unwrap_or([].iter());

        OutNeighbors {
            edges,
            edge_map: &self.edges,
        }
    }

    fn in_neighbors(&self, v: VertexHandle) -> Self::InNeighbors<'_> {
        let edges = self.in_edges.get(&v).map(|v| v.iter()).unwrap_or([].iter());

        InNeighbors {
            edges,
            edge_map: &self.edges,
        }
    }

    fn adjacent(&self, v: VertexHandle) -> Self::Adjacent<'_> {
        let out_edges = self
            .out_edges
            .get(&v)
            .map(|v| v.iter())
            .unwrap_or([].iter());
        let in_edges = self.in_edges.get(&v).map(|v| v.iter()).unwrap_or([].iter());

        Adjacent {
            out_edges,
            in_edges,
            edge_map: &self.edges,
        }
    }
}

impl TopologyMut for HashMapTopology {
    fn add_vertex(&mut self, handle: VertexHandle) -> bool {
        if !self.vertices.insert(handle) {
            return false;
        }

        self.out_edges.entry(handle).or_default();
        self.in_edges.entry(handle).or_default();

        true
    }

    fn add_edge(&mut self, handle: EdgeHandle, source: VertexHandle, target: VertexHandle) -> bool {
        if self.edges.contains_key(&handle) {
            return false;
        }

        self.edges.insert(handle, (source, target));
        self.out_edges.entry(source).or_default().push(handle);
        self.in_edges.entry(target).or_default().push(handle);

        true
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> bool {
        let Some((source, target)) = self.edges.remove(&handle) else {
            return false;
        };

        if let Some(vertex) = self.out_edges.get_mut(&source) {
            vertex.retain(|vx| *vx != handle);
        }

        if let Some(vertex) = self.in_edges.get_mut(&target) {
            vertex.retain(|vx| *vx != handle);
        }

        true
    }

    fn remove_vertex(&mut self, handle: VertexHandle) -> bool {
        if !self.vertices.remove(&handle) {
            return false;
        }

        if let Some(edges) = self.out_edges.remove(&handle) {
            for edge in edges {
                self.remove_edge(edge);
            }
        }

        if let Some(edges) = self.in_edges.remove(&handle) {
            for edge in edges {
                self.remove_edge(edge);
            }
        }

        true
    }
}

pub struct OutNeighbors<'a> {
    edges: std::slice::Iter<'a, EdgeHandle>,
    edge_map: &'a HashMap<EdgeHandle, (VertexHandle, VertexHandle)>,
}

impl<'a> Iterator for OutNeighbors<'a> {
    type Item = VertexHandle;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(edge) = self.edges.next() {
            if let Some((_, target)) = self.edge_map.get(edge) {
                return Some(*target);
            }
        }

        None
    }
}

pub struct InNeighbors<'a> {
    edges: std::slice::Iter<'a, EdgeHandle>,
    edge_map: &'a HashMap<EdgeHandle, (VertexHandle, VertexHandle)>,
}

impl<'a> Iterator for InNeighbors<'a> {
    type Item = VertexHandle;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(edge) = self.edges.next() {
            if let Some((source, _)) = self.edge_map.get(edge) {
                return Some(*source);
            }
        }

        None
    }
}

pub struct Adjacent<'a> {
    out_edges: std::slice::Iter<'a, EdgeHandle>,
    in_edges: std::slice::Iter<'a, EdgeHandle>,
    edge_map: &'a HashMap<EdgeHandle, (VertexHandle, VertexHandle)>,
}

impl<'a> Iterator for Adjacent<'a> {
    type Item = (VertexHandle, EdgeHandle);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(edge) = self.out_edges.next() {
            if let Some((_, target)) = self.edge_map.get(edge) {
                return Some((*target, *edge));
            }
        }

        while let Some(edge) = self.in_edges.next() {
            if let Some((source, _)) = self.edge_map.get(edge) {
                return Some((*source, *edge));
            }
        }

        None
    }
}
