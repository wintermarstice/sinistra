use crate::graph::Handle;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Checked<T: Handle>(T);

impl<T: Handle> Checked<T> {
    pub fn generation(handle: T) -> Option<Self> {
        if handle.generation() == 1 {
            return Some(Self(handle));
        }

        None
    }

    pub fn graph_and_generation(handle: T, graph: u32) -> Option<Self> {
        if handle.graph() == graph {
            return Self::generation(handle);
        }

        None
    }

    pub fn index(&self) -> usize {
        self.0.index()
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}
