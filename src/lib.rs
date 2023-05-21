//! A highly optimized [octree](https://en.wikipedia.org/wiki/Octree) implementation, with threading enabled for improved efficiency. This octree implementation is also capable of tracking highly dynamic environment.

pub mod point;
/// Calculates the location of your object in a 3d space.
pub trait Locatable {
    fn get_location(&self) -> [f32; 3];
}

#[derive(Debug)]
pub struct Octree<'tree_element, L: Locatable> {
    /// Put references to tree elements in a vec.
    pub points: &'tree_element Vec<L>,

    /// Use two points to define space bound.
    pub top_right_front: [f32; 3],
    pub bottom_left_back: [f32; 3],
}

impl<'tree_element, L: Locatable> Octree<'tree_element, L> {
    /// Build an [Octree] instance from a list of points.
    /// # Example
    /// ```rust
    /// use octree::point::Point3D;
    /// use octree::{Octree, Locatable};
    /// let points: Vec<Point3D> = Vec::new();
    /// let octree = Octree::new(&points);
    /// ```
    pub fn new(points: &'tree_element Vec<L>) -> Self {
        let top_right_front = if points.is_empty() {
            [f32::MAX, f32::MAX, f32::MAX]
        } else {
            // Dummy for now.
            [f32::MAX - 1.0, f32::MAX, f32::MAX]
        };
        let bottom_left_back = if points.is_empty() {
            [-f32::MAX, -f32::MAX, -f32::MAX]
        } else {
            // Dummy for now.
            [-f32::MAX + 1.0, -f32::MAX, -f32::MAX]
        };

        Self {
            points,
            top_right_front,
            bottom_left_back,
        }
    }

    /// Getter, returns a reference to the list which holds actual tree points data.
    pub fn points(&self) -> &Vec<L> {
        self.points
    }

    /// Getter, returns a reference to the positive boundary.
    pub fn top_right_front(&self) -> &[f32; 3] {
        &self.top_right_front
    }

    /// Getter, returns a reference to the negative boundary.
    pub fn bottom_left_back(&self) -> &[f32; 3] {
        &self.bottom_left_back
    }
}

#[cfg(test)]
mod tests {
    use super::point::Point3D;
    use super::*;

    #[test]
    /// Should be able to get 3D location for anything implements Locatable trait.
    fn location_trait() {
        let point = Point3D::default();

        assert_eq!(point.get_location(), [0.0, 0.0, 0.0]);
    }

    #[test]
    /// Should successfully build an octree with default boundaries if from empty point list.
    fn build_default_octree_from_empty_points() {
        let points: Vec<Point3D> = Vec::new();
        let octree = Octree::new(&points);

        assert_eq!(octree.points(), &points);
        assert_eq!(octree.top_right_front(), &[f32::MAX, f32::MAX, f32::MAX]);
        assert_eq!(
            octree.bottom_left_back(),
            &[-f32::MAX, -f32::MAX, -f32::MAX]
        );
    }
}
