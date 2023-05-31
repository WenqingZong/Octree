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
pub struct TreeNode<'point, L> {
    children: Option<[Box<TreeNode<'point, L>>; 8]>,
    bounding_box: BoundingBox,
    points: Vec<&'point L>,
    capacity: usize,
    splitted: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
    min: [f32; 3],
    max: [f32; 3],
}

impl<'point, L> Octree<'point, L>
where
    L: Locatable,
{
    pub fn new(points: Vec<&L>) -> Self {
        todo!()
        // Self {
        //     root: Some(Box::new(TreeNode::default()))
        // }
    }

    pub fn insert(&mut self, point: &L) {}

    pub fn delete(&mut self, point: &L) {}

    pub fn query(&self, bounding_box: &BoundingBox) -> Vec<&L> {
        todo!()
    }

    pub fn contain(&self, point: &L) -> bool {
        todo!();
    }
}

impl<'point, L> Default for Octree<'point, L>
where
    L: Locatable,
{
    fn default() -> Self {
        Self { root: None }
    }
}

impl<'point, L> TreeNode<'point, L>
where
    L: Locatable,
{
    pub fn new(points: Vec<&'point L>) -> Self {
        let mut tree_node: TreeNode<L> = TreeNode {
            // So the created bounding box contains every point in points.
            bounding_box: BoundingBox::new(points.clone()),
            ..Default::default()
        };

        for point in points {
            tree_node.insert(point);
        }

        tree_node
    }

    fn insert(&mut self, point: &'point L) -> bool {
        if !self.contains(point) {
            return false;
        }
        if self.points.len() < self.capacity {
            self.points.push(point);
            true
        } else {
            if !self.splitted {
                self.split();
            }

            assert!(self.children.is_some());
            for child in self.children.as_mut().unwrap().iter_mut() {
                let child = child.as_mut();
                if child.insert(point) {
                    return true;
                }
            }
            false
        }
    }

    fn split(&mut self) {
        assert!(self.children.is_none());
        self.splitted = true;
        // children: Option<[Box<TreeNode<'point, L>>; 8]>,
        let splitted_bounding_boxes = self.bounding_box.split();

        let mut children = [
            Box::new(TreeNode::default()),
            Box::new(TreeNode::default()),
            Box::new(TreeNode::default()),
            Box::new(TreeNode::default()),
            Box::new(TreeNode::default()),
            Box::new(TreeNode::default()),
            Box::new(TreeNode::default()),
            Box::new(TreeNode::default()),
        ];

        for (i, splitted_bounding_box) in splitted_bounding_boxes.iter().enumerate() {
            children[i].bounding_box = splitted_bounding_box.clone();
        }

        self.children = Some(children);
    }

    fn contains(&self, point: &L) -> bool {
        self.bounding_box.contains(&point.get_location())
    }
}

impl<'point, L> Default for TreeNode<'point, L>
where
    L: Locatable,
{
    fn default() -> Self {
        Self {
            children: None,
            bounding_box: BoundingBox::default(),
            points: Vec::new(),
            capacity: 8,
            splitted: false,
        }
    }
}

impl BoundingBox {
    pub fn new<L>(points: Vec<&L>) -> Self
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

    pub fn contains(&self, point: &[f32; 3]) -> bool {
        self.min[0] <= point[0]
            && point[0] < self.max[0]
            && self.min[1] <= point[1]
            && point[1] < self.max[1]
            && self.min[2] <= point[2]
            && point[2] < self.max[2]
    }

    pub fn overlaps(&self, other: &BoundingBox) -> bool {
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

    pub fn get_centre(&self) -> [f32; 3] {
        let mut ret = [0.0; 3];
        for (i, coordinate) in ret.iter_mut().enumerate() {
            *coordinate = (self.min[i] + self.max[i]) / 2.0;
        }
        ret
    }

    pub fn get_min(&self) -> &[f32; 3] {
        &self.min
    }

    pub fn get_max(&self) -> &[f32; 3] {
        &self.max
    }

    pub fn split(&self) -> [Self; 8] {
        let centre = self.get_centre();
        let min = self.min;
        let max = self.max;
        let mut ret = [
            Self::default(),
            Self::default(),
            Self::default(),
            Self::default(),
            Self::default(),
            Self::default(),
            Self::default(),
            Self::default(),
        ];
        // u: up, d: down, f: front, b: back, l: left, r: right.
        // example, ulb is the left back corner in the upper layer.
        // order: dfl, dfr, dbl, dbr, ufl, ufr, ubl, ubr.

        // dfl
        ret[0].min = min;
        ret[0].max = centre;

        // dfr
        ret[1].min = [centre[0], min[1], min[2]];
        ret[1].max = [max[0], centre[1], centre[2]];

        // dbl
        ret[2].min = [min[0], centre[1], min[2]];
        ret[2].max = [centre[0], max[1], centre[2]];

        // dbr
        ret[3].min = [centre[0], centre[1], min[2]];
        ret[3].max = [max[0], max[1], centre[2]];

        // ufl
        ret[4].min = [min[0], min[1], centre[2]];
        ret[4].max = [centre[0], centre[1], max[2]];

        // ufr
        ret[5].min = [centre[0], min[1], centre[2]];
        ret[5].max = [max[0], centre[1], max[2]];

        // ubl
        ret[6].min = [min[0], centre[1], centre[2]];
        ret[6].max = [centre[0], max[1], max[2]];

        // ubr
        ret[7].min = centre;
        ret[7].max = max;

        ret
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min: [f32::MAX, f32::MAX, f32::MAX],
            max: [f32::MIN, f32::MIN, f32::MIN],
        }
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
        let bounding_box = BoundingBox::new(vec![point1, point2, point3].iter().collect());

        assert_eq!(bounding_box.min, [0.0, -1.0, 0.0]);
        assert_eq!(bounding_box.max, [10.0, 0.0, 5.0]);
    }

    #[test]
    fn test_bounding_box_contains() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let bounding_box = BoundingBox::new(vec![point1, point2].iter().collect());
        let point3 = Point3D::new(5.0, 5.0, 5.0);
        let point4 = Point3D::new(10.0, 11.0, 9.0);

        assert!(bounding_box.contains(&point3.get_location()));
        assert!(!bounding_box.contains(&point4.get_location()));
    }

    #[test]
    fn test_bounding_box_overlaps() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let bounding_box1 = BoundingBox::new(vec![point1, point2].iter().collect());

        let point3 = Point3D::new(1.0, 1.0, 1.0);
        let point4 = Point3D::new(11.0, 11.0, 11.0);
        let bounding_box2 = BoundingBox::new(vec![point3, point4].iter().collect());

        let point5 = Point3D::new(1.0, 1.0, 1.0);
        let point6 = Point3D::new(9.0, 9.0, 9.0);
        let bounding_box3 = BoundingBox::new(vec![point5, point6].iter().collect());

        let point7 = Point3D::new(11.0, 0.0, 0.0);
        let point8 = Point3D::new(20.0, 20.0, 20.0);
        let bounding_box4 = BoundingBox::new(vec![point7, point8].iter().collect());

        assert!(bounding_box1.overlaps(&bounding_box2));
        assert!(bounding_box1.overlaps(&bounding_box3));
        assert!(!bounding_box1.overlaps(&bounding_box4));
    }

    #[test]
    fn test_bounding_box_centre() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let bounding_box = BoundingBox::new(vec![point1, point2].iter().collect());

        assert_eq!(bounding_box.get_centre(), [5.0; 3]);
    }

    #[test]
    fn test_bounding_box_split() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let bounding_box = BoundingBox::new(vec![point1, point2].iter().collect());

        let splitted = bounding_box.split();

        // u: up, d: down, f: front, b: back, l: left, r: right.
        // example, ulb is the left back corner in the upper layer.
        // order: dfl, dfr, dbl, dbr, ufl, ufr, ubl, ubr.

        // dfl
        assert_eq!(splitted[0].min, [0.0, 0.0, 0.0]);
        assert_eq!(splitted[0].max, [5.0, 5.0, 5.0]);

        // dfr
        assert_eq!(splitted[1].min, [5.0, 0.0, 0.0]);
        assert_eq!(splitted[1].max, [10.0, 5.0, 5.0]);

        // dbl
        assert_eq!(splitted[2].min, [0.0, 5.0, 0.0]);
        assert_eq!(splitted[2].max, [5.0, 10.0, 5.0]);

        // dbr
        assert_eq!(splitted[3].min, [5.0, 5.0, 0.0]);
        assert_eq!(splitted[3].max, [10.0, 10.0, 5.0]);

        // ufl
        assert_eq!(splitted[4].min, [0.0, 0.0, 5.0]);
        assert_eq!(splitted[4].max, [5.0, 5.0, 10.0]);

        // ufr
        assert_eq!(splitted[5].min, [5.0, 0.0, 5.0]);
        assert_eq!(splitted[5].max, [10.0, 5.0, 10.0]);

        // ubl
        assert_eq!(splitted[6].min, [0.0, 5.0, 5.0]);
        assert_eq!(splitted[6].max, [5.0, 10.0, 10.0]);

        // ubr
        assert_eq!(splitted[7].min, [5.0, 5.0, 5.0]);
        assert_eq!(splitted[7].max, [10.0, 10.0, 10.0]);
    }

    #[test]
    fn test_tree_node_construction_no_split() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let points = vec![point1.clone(), point2];
        let point_references: Vec<&Point3D> = points.iter().collect();
        let tree_node = TreeNode::new(point_references.clone());

        assert!(tree_node.children.is_none());
        assert_eq!(tree_node.bounding_box.min, [0.0, 0.0, 0.0]);
        assert_eq!(tree_node.bounding_box.max, [10.0, 10.0, 10.0]);
        assert_eq!(tree_node.points, vec![&point1]);
        assert_eq!(tree_node.capacity, 8);
        assert!(!tree_node.splitted);
    }

}
