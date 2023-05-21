//! A highly optimized octree implementation, with threading enabled for improved efficiency. This octree implementation is also capable of tracking highly dynamic environment.

/// Calculates the location of your object in a 3d space.
pub trait Locatable {
    fn get_location(&self) -> [f32; 3];
}

#[derive(Debug)]
pub struct Octree<'tree_element, L: Locatable> {
    /// Put references to tree elements in a vec.
    points: &'tree_element Vec<L>,

    /// Use two points to define space bound.
    top_right_front: [f32; 3],
    bottom_left_back: [f32; 3],
}

impl<'tree_element, L: Locatable> Octree<'tree_element, L> {
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

    pub fn points(&self) -> &Vec<L> {
        self.points
    }

    pub fn top_right_front(&self) -> &[f32; 3] {
        &self.top_right_front
    }

    pub fn bottom_left_back(&self) -> &[f32; 3] {
        &self.bottom_left_back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct Point3D {
        x: f32,
        y: f32,
        z: f32,
    }

    impl Locatable for Point3D {
        fn get_location(&self) -> [f32; 3] {
            [self.x, self.y, self.z]
        }
    }

    #[test]
    /// Should be able to get 3D location for anything implements Locatable trait.
    fn location_trait() {
        let point = Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

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
