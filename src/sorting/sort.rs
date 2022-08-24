use std::cmp::Ordering;

pub trait Sort {
    fn compare_resolution(&self, other: &Self) -> Ordering;
    fn compare_bandwidth(&self, other: &Self) -> Ordering;
}