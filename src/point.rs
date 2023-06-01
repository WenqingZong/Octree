/// The default point structure that this [Octree] holds.
use crate::Locatable;
#[derive(Clone, Debug, Default)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point3D {}

impl std::hash::Hash for Point3D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

impl Point3D {
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
    fn test_point3d_construction() {
        let point = Point3D::new(0.0, 1.0, 2.0);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 1.0);
        assert_eq!(point.z, 2.0);
    }

    #[test]
    fn test_point3d_locatable() {
        let point = Point3D::new(0.0, 1.0, 2.0);
        assert_eq!(point.get_location(), [0.0, 1.0, 2.0]);
    }

    #[test]
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
    fn test_point3d_hash() {
        let point1 = Point3D::new(0.0, 1.0, 2.0);
        let point2 = Point3D::new(0.0, 1.0, 2.0);
        let point3 = Point3D::new(0.0, 0.0, 0.0);

        let set: HashSet<&Point3D> = vec![&point1].into_iter().collect();
        assert!(set.contains(&point2));
        assert!(!set.contains(&point3));
    }
}
