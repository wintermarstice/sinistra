mod edge_topology;
mod endpoint_topology;
mod hash_map;
mod neighbor_topology;
mod undirected;
mod vertex_set;

pub use edge_topology::{EdgeTopology, EdgeTopologyMut};
pub use endpoint_topology::EndpointTopology;
pub use hash_map::HashMapTopology;
pub use neighbor_topology::NeighborTopology;
pub use undirected::Undirected;
pub use vertex_set::{VertexSet, VertexSetMut};

use crate::graph::{EdgeHandle, VertexHandle};

pub trait Topology: EndpointTopology {
    type Adjacent<'a>: Iterator<Item = (VertexHandle, EdgeHandle)>
    where
        Self: 'a;

    // TODO: Maybe we should default this to out_edges()?
    fn adjacent(&self, v: VertexHandle) -> Self::Adjacent<'_>;
}

pub trait TopologyMut: EdgeTopologyMut + Topology {}
