use crate::graph::{EdgeHandle, VertexHandle, topology::EdgeTopology};

pub trait EndpointTopology: EdgeTopology {
    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)>;
}
