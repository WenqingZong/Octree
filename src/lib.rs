//! A highly optimized [octree](https://en.wikipedia.org/wiki/Octree) implementation, with threading enabled for improved efficiency. This octree implementation is also capable of tracking highly dynamic environment.

pub mod point;
/// Calculates the location of your object in a 3d space.
pub trait Locatable {
    fn get_location(&self) -> [f32; 3];
}

#[derive(Debug)]
pub struct Octree<'point, L> {
    root: Option<Box<TreeNode<'point, L>>>,
}

#[derive(Debug)]
struct TreeNode<'point, L> {
    children: [Option<Box<TreeNode<'point, L>>>; 8],
    bounding_box: BoundingBox,
    points: Vec<&'point L>,
}

#[derive(Debug)]
struct BoundingBox {
    min: [f32; 3],
    max: [f32; 3],
}

impl<'point, L> Octree<'point, L> where L: Locatable {}

impl<'point, L> TreeNode<'point, L> where L: Locatable {}

impl BoundingBox {
    fn new<L>(points: &Vec<L>) -> Self
    where
        L: Locatable,
    {
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];

        for point in points {
            let location = point.get_location();
            for i in 0..3 {
                min[i] = min[i].min(location[i]);
                max[i] = max[i].max(location[i]);
            }
        }

        BoundingBox { min, max }
    }

    fn contains(&self, point: &[f32; 3]) -> bool {
        self.min[0] <= point[0]
            && point[0] < self.max[0]
            && self.min[1] <= point[1]
            && point[1] < self.max[1]
            && self.min[2] <= point[2]
            && point[2] < self.max[2]
    }

    fn overlaps(&self, other: &BoundingBox) -> bool {
        let other_point1 = other.min;
        let other_point2 = [other.min[0], other.min[1], other.max[2]];
        let other_point3 = [other.min[0], other.max[1], other.min[2]];
        let other_point4 = [other.min[0], other.max[1], other.max[2]];
        let other_point5 = [other.max[0], other.min[1], other.min[2]];
        let other_point6 = [other.max[0], other.min[1], other.max[2]];
        let other_point7 = [other.max[0], other.max[1], other.min[2]];
        let other_point8 = other.max;
        for point in [
            other_point1,
            other_point2,
            other_point3,
            other_point4,
            other_point5,
            other_point6,
            other_point7,
            other_point8,
        ] {
            if self.contains(&point) {
                return true;
            }
        }
        false
    }

}

#[cfg(test)]
mod tests {
    use super::point::Point3D;
    use super::*;

    #[test]
    fn test_bounding_box_construction() {
        let point1 = Point3D::new(10.0, 0.0, 0.0);
        let point2 = Point3D::new(0.0, -1.0, 0.0);
        let point3 = Point3D::new(0.0, 0.0, 5.0);
        let bounding_box = BoundingBox::new(&vec![point1, point2, point3]);

        assert_eq!(bounding_box.min, [0.0, -1.0, 0.0]);
        assert_eq!(bounding_box.max, [10.0, 0.0, 5.0]);
    }

    #[test]
    fn test_bounding_box_contains() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let bounding_box = BoundingBox::new(&vec![point1, point2]);
        let point3 = Point3D::new(5.0, 5.0, 5.0);
        let point4 = Point3D::new(10.0, 11.0, 9.0);

        assert!(bounding_box.contains(&point3.get_location()));
        assert!(!bounding_box.contains(&point4.get_location()));
    }

    #[test]
    fn test_bounding_box_overlaps() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let bounding_box1 = BoundingBox::new(&vec![point1, point2]);

        let point3 = Point3D::new(1.0, 1.0, 1.0);
        let point4 = Point3D::new(11.0, 11.0, 11.0);
        let bounding_box2 = BoundingBox::new(&vec![point3, point4]);

        let point5 = Point3D::new(1.0, 1.0, 1.0);
        let point6 = Point3D::new(9.0, 9.0, 9.0);
        let bounding_box3 = BoundingBox::new(&vec![point5, point6]);

        let point7 = Point3D::new(11.0, 0.0, 0.0);
        let point8 = Point3D::new(20.0, 20.0, 20.0);
        let bounding_box4 = BoundingBox::new(&vec![point7, point8]);

        assert!(bounding_box1.overlaps(&bounding_box2));
        assert!(bounding_box1.overlaps(&bounding_box3));
        assert!(!bounding_box1.overlaps(&bounding_box4));
    }
}
