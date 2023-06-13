//! The default, and a sample point structure that [Octree](crate::Octree) holds.
use std::hash::Hash;

use crate::Locatable;

/// Defines a basic 3D point. [Octree](crate::Octree) uses a [HashSet](std::collections::HashSet) to keep a record of
/// all points it has seen, so [PartialEq], [Eq]. and [Hash] must be defined as well.
#[derive(Clone, Debug, Default)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point3D {}

impl Hash for Point3D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

impl Point3D {
    /// Construct a 3D point from given coordination.
    /// # Example
    /// ```
    /// use octree::Locatable;
    /// use octree::point::Point3D;
    ///
    /// let point = Point3D::new(0.0, 0.0, 0.0);
    /// assert_eq!(point.get_location(), [0.0, 0.0, 0.0]);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3D { x, y, z }
    }
}

impl Locatable for Point3D {
    fn get_location(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    /// Should correctly construct a point from given coordinate.
    fn test_point3d_construction() {
        let point = Point3D::new(0.0, 1.0, 2.0);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 1.0);
        assert_eq!(point.z, 2.0);
    }

    #[test]
    /// Should be locatable.
    fn test_point3d_locatable() {
        let point = Point3D::new(0.0, 1.0, 2.0);
        assert_eq!(point.get_location(), [0.0, 1.0, 2.0]);
    }

    #[test]
    /// Should be able to determine if two points are equal, e.g., they have the same coordination.
    fn test_point3d_equal() {
        let point1 = Point3D::new(0.0, 1.0, 2.0);
        let point2 = Point3D::new(0.0, 1.0, 2.0);
        let point3 = Point3D::new(0.0, 0.0, 0.0);

        assert!(point1.eq(&point2));
        assert_eq!(&point1, &point2);

        assert!(point1.ne(&point3));
        assert_ne!(&point2, &point3);
    }

    #[test]
    /// Should be able to be stored in a [HashSet]
    fn test_point3d_hash() {
        let point1 = Point3D::new(0.0, 1.0, 2.0);
        let point2 = Point3D::new(0.0, 1.0, 2.0);
        let point3 = Point3D::new(0.0, 0.0, 0.0);

        let set: HashSet<&Point3D> = vec![&point1].into_iter().collect();
        assert!(set.contains(&point2));
        assert!(!set.contains(&point3));
    }
}
