use std::ops::Add;

pub trait Weight: Copy + Ord + Add<Output = Self> + Default {
    fn infinity() -> Self;
}

pub trait Weighted {
    type Weight: Weight;
    fn weight(&self) -> Self::Weight;
}
