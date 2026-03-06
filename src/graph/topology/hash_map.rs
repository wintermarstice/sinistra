use std::{
    collections::{HashMap, HashSet},
    iter::Copied,
};

use crate::graph::{
    Checked, EdgeHandle, EdgeTopology, EdgeTopologyMut, EndpointTopology, NeighborTopology,
    Topology, TopologyMut, VertexHandle, VertexSet, VertexSetMut,
};

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

    fn checked_vertex(&self, handle: VertexHandle) -> Option<Checked<VertexHandle>> {
        let checked = Checked::generation(handle)?;
        if !self.vertices.contains(&checked.into_inner()) {
            return None;
        }

        Some(checked)
    }

    fn checked_edge(&self, handle: EdgeHandle) -> Option<Checked<EdgeHandle>> {
        let checked = Checked::generation(handle)?;
        if !self.edges.contains_key(&checked.into_inner()) {
            return None;
        }

        Some(checked)
    }
}

impl VertexSet for HashMapTopology {
    type Vertices<'a>
        = Copied<std::collections::hash_set::Iter<'a, VertexHandle>>
    where
        Self: 'a;

    fn vertices(&self) -> Self::Vertices<'_> {
        self.vertices.iter().copied()
    }
}

impl NeighborTopology for HashMapTopology {
    type OutNeighbors<'a>
        = OutNeighbors<'a>
    where
        Self: 'a;

    type InNeighbors<'a>
        = InNeighbors<'a>
    where
        Self: 'a;

    fn out_neighbors(&self, v: VertexHandle) -> Self::OutNeighbors<'_> {
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => {
                return OutNeighbors {
                    edges: [].iter(),
                    edge_map: &self.edges,
                };
            }
        };

        let edges = self
            .out_edges
            .get(&checked.into_inner())
            .map(|v| v.iter())
            .unwrap_or([].iter());

        OutNeighbors {
            edges,
            edge_map: &self.edges,
        }
    }

    fn in_neighbors(&self, v: VertexHandle) -> Self::InNeighbors<'_> {
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => {
                return InNeighbors {
                    edges: [].iter(),
                    edge_map: &self.edges,
                };
            }
        };

        let edges = self
            .in_edges
            .get(&checked.into_inner())
            .map(|v| v.iter())
            .unwrap_or([].iter());

        InNeighbors {
            edges,
            edge_map: &self.edges,
        }
    }
}

impl EdgeTopology for HashMapTopology {
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

    fn edges(&self) -> Self::Edges<'_> {
        self.edges.keys().copied()
    }

    fn out_edges(&self, v: VertexHandle) -> Self::OutEdges<'_> {
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => return [].iter().copied(),
        };

        self.out_edges
            .get(&checked.into_inner())
            .map(|v| v.iter())
            .unwrap_or([].iter())
            .copied()
    }

    fn in_edges(&self, v: VertexHandle) -> Self::InEdges<'_> {
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => return [].iter().copied(),
        };

        self.in_edges
            .get(&checked.into_inner())
            .map(|v| v.iter())
            .unwrap_or([].iter())
            .copied()
    }
}

impl EndpointTopology for HashMapTopology {
    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        let checked = Checked::generation(edge)?;
        self.edges.get(&checked.into_inner()).copied()
    }
}

impl Topology for HashMapTopology {
    type Adjacent<'a>
        = Adjacent<'a>
    where
        Self: 'a;

    fn adjacent(&self, v: VertexHandle) -> Self::Adjacent<'_> {
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => {
                return Adjacent {
                    out_edges: [].iter(),
                    in_edges: [].iter(),
                    edge_map: &self.edges,
                };
            }
        };

        let out_edges = self
            .out_edges
            .get(&checked.into_inner())
            .map(|v| v.iter())
            .unwrap_or([].iter());
        let in_edges = self
            .in_edges
            .get(&checked.into_inner())
            .map(|v| v.iter())
            .unwrap_or([].iter());

        Adjacent {
            out_edges,
            in_edges,
            edge_map: &self.edges,
        }
    }
}

impl VertexSetMut for HashMapTopology {
    fn add_vertex(&mut self, handle: VertexHandle) -> bool {
        let Some(checked) = Checked::generation(handle) else {
            return false;
        };

        if !self.vertices.insert(checked.into_inner()) {
            return false;
        }

        self.out_edges.entry(handle).or_default();
        self.in_edges.entry(handle).or_default();

        true
    }

    fn remove_vertex(&mut self, handle: VertexHandle) -> bool {
        let checked = match self.checked_vertex(handle) {
            Some(checked) => checked,
            None => return false,
        };

        if !self.vertices.remove(&checked.into_inner()) {
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

impl EdgeTopologyMut for HashMapTopology {
    fn add_edge(&mut self, handle: EdgeHandle, source: VertexHandle, target: VertexHandle) -> bool {
        let Some(checked_handle) = Checked::generation(handle) else {
            return false;
        };
        let Some(checked_source) = Checked::generation(source) else {
            return false;
        };
        let Some(checked_target) = Checked::generation(target) else {
            return false;
        };

        if !self.vertices.contains(&checked_source.into_inner())
            || !self.vertices.contains(&checked_target.into_inner())
            || self.edges.contains_key(&checked_handle.into_inner())
        {
            return false;
        }

        self.edges.insert(handle, (source, target));
        self.out_edges.entry(source).or_default().push(handle);
        self.in_edges.entry(target).or_default().push(handle);

        true
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> bool {
        let checked = match self.checked_edge(handle) {
            Some(checked) => checked,
            None => return false,
        };

        let Some((source, target)) = self.edges.remove(&checked.into_inner()) else {
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
}

impl TopologyMut for HashMapTopology {}

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
