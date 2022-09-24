//! A basic kd-tree implementation used for determining a query point's nearest neighbour.
//! Eventually this code might be moved to an external crate, or replaced by a better
//! implementation.

use std::cmp::Ordering;

/// An object in euclidean space
pub trait VectorObject {
    type ScalarType: PartialOrd + Clone;

    /// Return the number of dimensions of the object
    fn num_dimensions(&self) -> usize;

    /// Return the coordinate along a particular axis
    fn coordinate(&self, axis: usize) -> Self::ScalarType;

    /// Return the distance squared between two objects
    fn dist_sqr(&self, other: &Self) -> Self::ScalarType;

    /// Return the squared distance between two objects along a particular axis
    fn dist_axis(&self, other: &Self, axis: usize) -> Self::ScalarType;
}

impl VectorObject for (f32, f32) {
    type ScalarType = f32;

    fn num_dimensions(&self) -> usize {
        2
    }

    fn coordinate(&self, axis: usize) -> f32 {
        match axis {
            0 => self.0,
            1 => self.1,
            _ => panic!("Index out of bounds"),
        }
    }

    fn dist_sqr(&self, other: &Self) -> f32 {
        (self.0 - other.0).powf(2.0) + (self.1 - other.1).powf(2.0)
    }

    fn dist_axis(&self, other: &Self, axis: usize) -> f32 {
        let dist = (self.coordinate(axis) - other.coordinate(axis)).abs();
        dist * dist
    }
}

fn coordinate_compare<T>(a: &T, b: &T, axis: usize) -> Option<Ordering>
where
    T: VectorObject,
{
    a.coordinate(axis).partial_cmp(&b.coordinate(axis))
}

fn min_option_by<T, F>(a: Option<T>, b: Option<T>, cmp: F) -> Option<T>
where
    F: FnOnce(&T, &T) -> bool,
{
    match (a, b) {
        (Some(a), Some(b)) => {
            if cmp(&a, &b) {
                Some(a)
            }
            else {
                Some(b)
            }
        }
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

enum KdTreeNode<T>
where
    T: VectorObject,
{
    Branch(Box<Branch<T>>),
    Leaf(Option<Box<T>>),
}

impl<T> KdTreeNode<T>
where
    T: VectorObject + Clone,
{
    fn find_nearest(&self, query: &T, best: Option<T>) -> Option<T> {
        match self {
            KdTreeNode::Branch(branch) => {
                let axis = branch.split_axis;
                let (near, far) = {
                    if query.coordinate(axis) < branch.split_val.coordinate(axis) {
                        (&branch.left, &branch.right)
                    }
                    else {
                        (&branch.right, &branch.left)
                    }
                };

                let best = near.find_nearest(query, best);
                let lower_bound = branch.split_val.dist_axis(query, axis);

                if best.as_ref().map(|x| lower_bound < x.dist_sqr(query)).unwrap_or(true) {
                    far.find_nearest(query, best)
                }
                else {
                    best
                }
            }
            KdTreeNode::Leaf(leaf) => {
                min_option_by(best, leaf.as_ref().map(|x| (**x).clone()), |a, b| {
                    a.dist_sqr(query) < b.dist_sqr(query)
                })
            }
        }
    }
}

struct Branch<T>
where
    T: VectorObject,
{
    split_axis: usize,
    split_val: T,
    left: KdTreeNode<T>,
    right: KdTreeNode<T>,
}

pub struct KdTree<T>
where
    T: VectorObject,
{
    root: KdTreeNode<T>,
}

impl<T> KdTree<T>
where
    T: VectorObject + Clone,
{
    /// Create a new Kd-tree given a slice of objects to include in the tree.
    ///
    /// This function modifies the order of the objects in the tree.
    pub fn new(objects: &mut [T]) -> Option<KdTree<T>> {
        if objects.is_empty() {
            return None;
        }

        let num_dims = objects[0].num_dimensions();
        Some(KdTree { root: kd_tree_builder(objects, 0, num_dims) })
    }

    /// Find the nearest point in the kd-tree to a given query point
    pub fn find_nearest(&self, query: &T) -> Option<T> {
        self.root.find_nearest(query, None)
    }
}

fn kd_tree_builder<T>(objects: &mut [T], axis: usize, num_dims: usize) -> KdTreeNode<T>
where
    T: VectorObject + Clone,
{
    match objects.len() {
        0 => KdTreeNode::Leaf(None),
        1 => KdTreeNode::Leaf(Some(Box::new(objects[0].clone()))),
        _ => {
            objects.sort_by(|a, b| {
                coordinate_compare(a, b, axis)
                    .expect("Cannot construct kd-tree with unorderable value.")
            });

            let median = objects.len() / 2;
            let split_val = objects[median].clone();
            let (left, right) = objects.split_at_mut(median);

            let branch = Branch {
                split_axis: axis,
                split_val,
                left: kd_tree_builder(left, (axis + 1) % num_dims, num_dims),
                right: kd_tree_builder(right, (axis + 1) % num_dims, num_dims),
            };

            KdTreeNode::Branch(Box::new(branch))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let mut points = [(2.0, 3.0), (5.0, 4.0), (9.0, 6.0), (4.0, 7.0), (8.0, 1.0), (7.0, 2.0)];
        let kd_tree = KdTree::new(&mut points).unwrap();

        assert_eq!(kd_tree.find_closest(&(1.0, 1.0)), Some((2.0, 3.0)));
        assert_eq!(kd_tree.find_closest(&(2.0, 3.0)), Some((2.0, 3.0)));
        assert_eq!(kd_tree.find_closest(&(5.0, 4.0)), Some((5.0, 4.0)));
        assert_eq!(kd_tree.find_closest(&(9.0, 6.0)), Some((9.0, 6.0)));
        assert_eq!(kd_tree.find_closest(&(4.0, 7.0)), Some((4.0, 7.0)));
        assert_eq!(kd_tree.find_closest(&(8.0, 1.0)), Some((8.0, 1.0)));
        assert_eq!(kd_tree.find_closest(&(7.0, 2.0)), Some((7.0, 2.0)));
    }
}
