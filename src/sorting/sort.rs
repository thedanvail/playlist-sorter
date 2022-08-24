use std::cmp::Ordering;

/// A trait that extracts some of the `std::cmp::Ordering` functionality so we can bind it
/// to the implementing structs as we need them.
pub trait Sort {
    fn compare_resolution(&self, other: &Self) -> Ordering;
    fn compare_bandwidth(&self, other: &Self) -> Ordering;
}