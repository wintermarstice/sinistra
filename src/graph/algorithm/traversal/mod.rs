mod bfs;
mod dfs;
mod dijkstra;

pub use bfs::Event as BfsEvent;
pub use bfs::{bfs, bfs_tree_edges, bfs_vertices};

pub use dfs::Event as DfsEvent;
pub use dfs::dfs;

pub use dijkstra::Dijkstra;
pub use dijkstra::Event as DijkstraEvent;
pub use dijkstra::dijkstra;
pub use dijkstra::dijkstra_distances;

use std::collections::{BinaryHeap, VecDeque};

use crate::graph::{EdgeHandle, VertexHandle};

#[derive(Debug, Clone, Copy)]
pub enum TraversalEvent {
    Discover {
        vertex: VertexHandle,
    },
    Examine {
        source: VertexHandle,
        target: VertexHandle,
        edge: EdgeHandle,
    },
    Finish {
        vertex: VertexHandle,
    },
}

pub trait Frontier<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
}

impl<T> Frontier<T> for VecDeque<T> {
    fn push(&mut self, value: T) {
        self.push_back(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }
}

impl<T> Frontier<T> for Vec<T> {
    fn push(&mut self, value: T) {
        self.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T: Ord> Frontier<T> for BinaryHeap<T> {
    fn push(&mut self, value: T) {
        BinaryHeap::push(self, value);
    }

    fn pop(&mut self) -> Option<T> {
        BinaryHeap::pop(self)
    }
}

pub trait Policy {
    type Event;
    type Item;

    fn start(&mut self, start: VertexHandle);
    fn pop(&mut self) -> Option<Self::Item>;
    fn process(&mut self, item: Self::Item, pending: &mut VecDeque<Self::Event>);
}

pub struct Traversal<P: Policy> {
    policy: P,
    pending: VecDeque<P::Event>,
}

impl<P: Policy> Traversal<P> {
    pub fn new(mut policy: P, start: VertexHandle) -> Self {
        let pending = VecDeque::new();

        policy.start(start);

        Self { policy, pending }
    }
}

impl<P: Policy> Iterator for Traversal<P> {
    type Item = P::Event;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(event) = self.pending.pop_front() {
                return Some(event);
            }

            let item = self.policy.pop()?;
            self.policy.process(item, &mut self.pending);
        }
    }
}
