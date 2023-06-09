//! A highly optimized [octree](https://en.wikipedia.org/wiki/Octree) implementation, with threading enabled for improved efficiency. This octree implementation is also capable of tracking highly dynamic environment.

use std::collections::HashSet;
use std::hash::Hash;

pub mod point;
/// Calculates the location of your object in a 3d space. Your data structure must implement this trait so [Octree] can
/// can get location of each data point. A sample implementation is provided in [Point3D](point::Point3D).
pub trait Locatable {
    fn get_location(&self) -> [f32; 3];
}

/// The [Octree](https://en.wikipedia.org/wiki/Octree) data structure.
#[derive(Debug)]
pub struct Octree<'point, L> {
    root: TreeNode<'point, L>,
}

/// Recursive data structure for tree node. Its children is None if not splitted. If splitted, then its children is an
/// array of length 8 of (points of) sub tree nodes.
#[derive(Debug)]
struct TreeNode<'point, L> {
    children: Option<[Box<TreeNode<'point, L>>; 8]>,
    bounding_box: BoundingBox,
    points: HashSet<&'point L>,
    capacity: usize,
    splitted: bool,
}

/// Bounding box defines a 3D space.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
    min: [f32; 3],
    max: [f32; 3],
}

impl<'point, L> Octree<'point, L>
where
    L: Locatable + Eq + Hash,
{
    /// Construct an [Octree] that covers all given points.
    /// # Example
    /// ```
    /// use octree::point::Point3D;
    /// use octree::Octree;
    ///
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    ///
    /// // Note points takes ownership of the above two points.
    /// let points = vec![point1, point2];
    ///
    /// let octree = Octree::new(points.iter().collect());
    /// ```
    pub fn new(points: Vec<&'point L>) -> Self {
        Self {
            root: TreeNode::new(points),
        }
    }

    /// Insert a new point. If [Octree] does not cover the new point then nothing will change.
    /// # Example
    /// ```
    /// use octree::point::Point3D;
    /// use octree::Octree;
    ///
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    /// let point3 = Point3D::new(5.0, 5.0, 5.0);
    /// let point4 = Point3D::new(20.0, 20.0, 20.0);
    ///
    /// // Note points takes ownership of the above two points.
    /// let points = vec![point1, point2];
    /// let mut octree = Octree::new(points.iter().collect());
    ///
    /// assert!(octree.insert(&point3));
    /// assert!(!octree.insert(&point4));
    /// ```
    pub fn insert(&mut self, point: &'point L) -> bool {
        self.root.insert(point)
    }

    /// Delete a point from current [Octree], if the point is not in the tree, then nothing will change.
    /// # Example
    /// ```
    /// use octree::point::Point3D;
    /// use octree::Octree;
    ///
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    /// let point3 = Point3D::new(20.0, 20.0, 20.0);
    ///
    /// let points = vec![point1.clone(), point2];
    /// let mut octree = Octree::new(points.iter().collect());
    ///
    /// assert!(octree.delete(&point1));
    /// assert!(!octree.delete(&point3));
    /// ```
    pub fn delete(&mut self, point: &'point L) -> bool {
        self.root.delete(point)
    }

    /// Find all points covered by a specified [BoundingBox].
    /// # Example
    /// ```
    /// use std::collections::HashSet;
    ///
    /// use octree::point::Point3D;
    /// use octree::{BoundingBox, Octree};
    ///
    /// // Build an example octree.
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    /// let point3 = Point3D::new(4.0, 4.0, 4.0);
    /// let points = vec![point1.clone(), point2];
    /// let mut octree = Octree::new(points.iter().collect());
    /// let point4 = Point3D::new(5.0, 10.0, 5.0);
    /// octree.insert(&point3);
    /// octree.insert(&point4);
    ///
    /// let points_for_query = vec![point1.clone(), point4.clone()];
    /// let bounding_box = BoundingBox::new(points_for_query.iter().collect());
    ///
    /// assert_eq!(
    ///     // The actual query.
    ///     octree.query(&bounding_box),
    ///     HashSet::from([&point1, &point3])
    /// );
    /// ```
    pub fn query(&self, bounding_box: &BoundingBox) -> HashSet<&L> {
        self.root.query(bounding_box)
    }

    /// Check if a point is already recorded.
    /// # Example
    /// ```
    /// use octree::point::Point3D;
    /// use octree::Octree;
    ///
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    /// let point3 = Point3D::new(20.0, 20.0, 20.0);
    ///
    /// let points = vec![point1.clone(), point2];
    ///
    /// let octree = Octree::new(points.iter().collect());
    ///
    /// assert!(octree.contains(&point1));
    /// assert!(!octree.contains(&point3));
    /// ```
    pub fn contains(&self, point: &L) -> bool {
        self.root.contains(point)
    }

    /// Check if a point can be covered by the current [Octree].
    /// # Example
    /// ```
    /// use octree::point::Point3D;
    /// use octree::Octree;
    ///
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    /// let point3 = Point3D::new(5.0, 5.0, 5.0);
    /// let point4 = Point3D::new(20.0, 20.0, 20.0);
    ///
    /// // Note points takes ownership of the above two points.
    /// let points = vec![point1, point2];
    ///
    /// let octree = Octree::new(points.iter().collect());
    ///
    /// assert!(octree.covers(&point3));
    /// assert!(!octree.covers(&point4));
    /// ```
    pub fn covers(&self, point: &L) -> bool {
        self.root.covers(point)
    }

    /// Check if the space occupied by current [Octree] overlap with a given [BoundingBox].
    /// # Example
    /// ```
    /// use octree::point::Point3D;
    /// use octree::{BoundingBox, Octree};
    ///
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    /// let points = vec![point1, point2];
    /// let octree = Octree::new(points.iter().collect());
    ///
    /// let point3 = Point3D::new(1.0, 1.0, 1.0);
    /// let point4 = Point3D::new(11.0, 11.0, 11.0);
    /// let bounding_box1 = BoundingBox::new(vec![point3, point4].iter().collect());
    ///
    /// let point5 = Point3D::new(1.0, 1.0, 1.0);
    /// let point6 = Point3D::new(9.0, 9.0, 9.0);
    /// let bounding_box2 = BoundingBox::new(vec![point5, point6].iter().collect());
    ///
    /// let point7 = Point3D::new(11.0, 0.0, 0.0);
    /// let point8 = Point3D::new(20.0, 20.0, 20.0);
    /// let bounding_box3 = BoundingBox::new(vec![point7, point8].iter().collect());
    ///
    /// assert!(octree.overlaps(&bounding_box1));
    /// assert!(octree.overlaps(&bounding_box2));
    /// assert!(!octree.overlaps(&bounding_box3));
    /// ```
    pub fn overlaps(&self, bounding_box: &BoundingBox) -> bool {
        self.root.overlaps(bounding_box)
    }
}

impl<'point, L> Default for Octree<'point, L>
where
    L: Locatable + Eq + Hash,
{
    fn default() -> Self {
        Self {
            root: TreeNode::default(),
        }
    }
}

// Implement PartialEq and Eq to make testing easier.
impl<'point, L> PartialEq for Octree<'point, L>
where
    L: Locatable + Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<'point, L> Eq for Octree<'point, L> where L: Locatable + Eq + Hash {}

// Implement PartialEq and Eq to make testing easier.
impl<'point, L> PartialEq for TreeNode<'point, L>
where
    L: Locatable + Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
            && self.bounding_box == other.bounding_box
            && self.points == other.points
            && self.capacity == other.capacity
            && self.splitted == other.splitted
    }
}

impl<'point, L> Eq for TreeNode<'point, L> where L: Locatable + Eq + Hash {}

impl<'point, L> TreeNode<'point, L>
where
    L: Locatable + Eq + Hash,
{
    fn new(points: Vec<&'point L>) -> Self {
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
        // Do nothing is the point won't be covered by current node.
        if !self.covers(point) {
            return false;
        }
        // Yes, we have enough remaining space.
        if self.points.len() < self.capacity {
            self.points.insert(point);
            true
        } else {
            // Already have 8 points, should split further.
            if !self.splitted {
                self.split();
            }

            // Ask all children: does this point belongs to you?
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
        self.splitted = true;

        let splitted_bounding_boxes = self.bounding_box.split();

        // Place holder for the 8 new nodes.
        let mut children = [
            Box::<TreeNode<'_, L>>::default(),
            Box::<TreeNode<'_, L>>::default(),
            Box::<TreeNode<'_, L>>::default(),
            Box::<TreeNode<'_, L>>::default(),
            Box::<TreeNode<'_, L>>::default(),
            Box::<TreeNode<'_, L>>::default(),
            Box::<TreeNode<'_, L>>::default(),
            Box::<TreeNode<'_, L>>::default(),
        ];

        // Assign new bounding box to placeholders.
        for (i, splitted_bounding_box) in splitted_bounding_boxes.iter().enumerate() {
            children[i].bounding_box = splitted_bounding_box.clone();
        }

        self.children = Some(children);
    }

    fn covers(&self, point: &L) -> bool {
        self.bounding_box.covers(&point.get_location())
    }

    fn contains(&self, point: &L) -> bool {
        self.points.contains(point)
    }

    fn delete(&mut self, point: &L) -> bool {
        // HashSet returns false if removes a non-existing element.
        let ret = self.points.remove(point);
        if ret {
            if let Some(children) = &mut self.children {
                for child in children.iter_mut() {
                    child.delete(point);
                }
            }
            // TODO: 7 children contain nothing and only one child contains some point, then re-merge them into 1 node
            // to reduce tree depth.
            // Possible performance improvement?
        }
        ret
    }

    fn query(&self, bounding_box: &BoundingBox) -> HashSet<&L> {
        // Place holder for the query answer.
        let mut ret = HashSet::new();

        // If they do not overlap, then we won't find any points in this sub tree which is covered by the query bounding
        // box.
        if !self.bounding_box.overlaps(bounding_box) {
            return ret;
        }
        for point in &self.points {
            if bounding_box.covers(&point.get_location()) {
                // point is of type &&L.
                ret.insert(*point);
            }
        }

        // Recursively ask sub tree if they have something covered by the query bounding box.
        if self.splitted {
            for child in self.children.as_ref().unwrap().iter() {
                ret.extend(child.query(bounding_box));
            }
        }
        ret
    }

    fn overlaps(&self, bounding_box: &BoundingBox) -> bool {
        self.bounding_box.overlaps(bounding_box)
    }
}

impl<'point, L> Default for TreeNode<'point, L>
where
    L: Locatable + Eq + Hash,
{
    fn default() -> Self {
        Self {
            children: None,
            bounding_box: BoundingBox::default(),
            points: HashSet::new(),
            capacity: 8,
            splitted: false,
        }
    }
}

impl BoundingBox {
    /// Construct a new [BoundingBox] which can hold all given points.
    /// # Example
    /// ```
    /// use octree::point::Point3D;
    /// use octree::BoundingBox;
    ///
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    /// let bounding_box = BoundingBox::new(vec![point1, point2].iter().collect());
    /// ```
    pub fn new<L>(points: Vec<&L>) -> Self
    where
        L: Locatable,
    {
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];

        // Linear search to find the min and max point.
        for point in points {
            let location = point.get_location();
            for i in 0..3 {
                min[i] = min[i].min(location[i]);
                max[i] = max[i].max(location[i]);
            }
        }

        BoundingBox { min, max }
    }

    /// Check if a point can be covered by this [BoundingBox].
    /// # Example
    /// ```
    /// use octree::point::Point3D;
    /// use octree::{BoundingBox, Locatable};
    ///
    /// let point1 = Point3D::new(0.0, 0.0, 0.0);
    /// let point2 = Point3D::new(10.0, 10.0, 10.0);
    /// let point3 = Point3D::new(5.0, 5.0, 5.0);
    /// let point4 = Point3D::new(20.0, 20.0, 20.0);
    /// let bounding_box = BoundingBox::new(vec![point1, point2].iter().collect());
    ///
    /// assert!(bounding_box.covers(&point3.get_location()));
    /// assert!(!bounding_box.covers(&point4.get_location()));
    /// ```
    pub fn covers(&self, point: &[f32; 3]) -> bool {
        self.min[0] <= point[0]
            && point[0] < self.max[0]
            && self.min[1] <= point[1]
            && point[1] < self.max[1]
            && self.min[2] <= point[2]
            && point[2] < self.max[2]
    }

    /// Check if two [BoundingBox]es overlap.
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
            if self.covers(&point) {
                return true;
            }
        }
        false
    }

    /// Getter for [BoundingBox] centre coordination.
    pub fn get_centre(&self) -> [f32; 3] {
        let mut ret = [0.0; 3];
        for (i, coordinate) in ret.iter_mut().enumerate() {
            *coordinate = (self.min[i] + self.max[i]) / 2.0;
        }
        ret
    }

    /// Getter for [BoundingBox] min corner coordination.
    pub fn get_min(&self) -> &[f32; 3] {
        &self.min
    }

    /// Getter for [BoundingBox] max corner coordination.
    pub fn get_max(&self) -> &[f32; 3] {
        &self.max
    }

    /// Split the [BoundingBox] into 8 sub [BoundingBox]es.
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
    /// Construct a default [BoundingBox], covers the whole space which can be represented by [f32].
    fn default() -> Self {
        Self {
            // Will be used in min() and max() function later so the initial value of min is f32::MAX.
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
    /// Should construct bounding box with the specified min and max corner.
    fn test_bounding_box_construction() {
        let point1 = Point3D::new(10.0, 0.0, 0.0);
        let point2 = Point3D::new(0.0, -1.0, 0.0);
        let point3 = Point3D::new(0.0, 0.0, 5.0);
        let bounding_box = BoundingBox::new(vec![point1, point2, point3].iter().collect());

        assert_eq!(bounding_box.get_min(), &[0.0, -1.0, 0.0]);
        assert_eq!(bounding_box.get_max(), &[10.0, 0.0, 5.0]);
    }

    #[test]
    /// Should identify if a point is covered by this area.
    /// Note that the bounding box covers min surface but does not cover max surface.
    fn test_bounding_box_covers() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let bounding_box = BoundingBox::new(vec![point1.clone(), point2.clone()].iter().collect());
        let point3 = Point3D::new(5.0, 5.0, 5.0);
        let point4 = Point3D::new(10.0, 11.0, 9.0);

        assert!(bounding_box.covers(&point1.get_location()));
        assert!(!bounding_box.covers(&point2.get_location()));
        assert!(bounding_box.covers(&point3.get_location()));
        assert!(!bounding_box.covers(&point4.get_location()));
    }

    #[test]
    /// Should identify if two bounding boxes overlaps / intersects.
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
    /// Should correctly calculate bounding box centre coordination.
    fn test_bounding_box_centre() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let bounding_box = BoundingBox::new(vec![point1, point2].iter().collect());

        assert_eq!(bounding_box.get_centre(), [5.0; 3]);
    }

    #[test]
    /// Should split the current bounding box into 8 smaller bounding boxes.
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
    /// Should construct a tree node with default settings.
    fn test_tree_node_default_construction() {
        let tree_node: TreeNode<Point3D> = TreeNode::default();
        assert!(tree_node.children.is_none());
        assert_eq!(tree_node.bounding_box, BoundingBox::default());
        assert_eq!(tree_node.points, HashSet::new());
        assert_eq!(tree_node.capacity, 8);
        assert!(!tree_node.splitted);
    }

    #[test]
    /// Should construct a tree node and do not further split.
    fn test_tree_node_construction_no_split() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let points = vec![point1.clone(), point2];
        let point_references: Vec<&Point3D> = points.iter().collect();
        let tree_node = TreeNode::new(point_references.clone());

        assert!(tree_node.children.is_none());
        assert_eq!(tree_node.bounding_box.min, [0.0, 0.0, 0.0]);
        assert_eq!(tree_node.bounding_box.max, [10.0, 10.0, 10.0]);
        assert_eq!(tree_node.points, HashSet::from([&point1]));
        assert_eq!(tree_node.capacity, 8);
        assert!(!tree_node.splitted);
    }

    #[test]
    /// Should construct an octree with default settings.
    fn test_octree_default_construction() {
        let octree: Octree<Point3D> = Octree::default();

        assert_eq!(octree.root, TreeNode::default());
    }

    #[test]
    /// Should construct an octree from some given points.
    fn test_octree_construction() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let points = vec![point1, point2];
        let octree = Octree::new(points.iter().collect());
        let tree_node = TreeNode::new(points.iter().collect());

        assert_eq!(octree.root, tree_node);
    }

    #[test]
    /// Should insert a point into octree if the octree covers it, and does nothing if not.
    fn test_octree_insert() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let point3 = Point3D::new(5.0, 5.0, 5.0);
        let points = vec![point1.clone(), point2.clone()];
        let mut octree1 = Octree::new(points.iter().collect());
        octree1.insert(&point3);
        let all_points = vec![point1, point2, point3.clone()];
        let octree2 = Octree::new(all_points.iter().collect());

        assert_eq!(octree1, octree2);

        let point4 = Point3D::new(20.0, 20.0, 20.0);
        octree1.insert(&point4);

        assert_eq!(octree1, octree2);
    }

    #[test]
    /// Should delete a point from octree if the point is recorded, and does nothing if not.
    fn test_octree_delete() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(5.0, 5.0, 5.0);
        let point3 = Point3D::new(10.0, 10.0, 10.0);
        let points = vec![point1.clone(), point3.clone()];
        let all_points = vec![point1, point2.clone(), point3];
        let mut octree1 = Octree::new(all_points.iter().collect());
        octree1.delete(&point2);
        let octree2 = Octree::new(points.iter().collect());

        assert_eq!(octree1, octree2);

        let point4 = Point3D::new(100.0, 100.0, 100.0);
        octree1.delete(&point4);

        assert_eq!(octree1, octree2);
    }

    #[test]
    /// Should correctly identify if a point can be covered by a octree.
    fn test_octree_covers() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(5.0, 5.0, 5.0);
        let points = vec![point1, point2];
        let octree = Octree::new(points.iter().collect());
        let point3 = Point3D::new(10.0, 10.0, 10.0);
        let point4 = Point3D::new(2.0, 2.0, 2.0);

        assert!(octree.covers(&point4));
        assert!(!octree.covers(&point3));
    }

    #[test]
    /// Should correctly identify if a point is recorded.
    fn test_octree_contains() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(5.0, 5.0, 5.0);
        let points = vec![point1.clone(), point2.clone()];
        let octree = Octree::new(points.iter().collect());
        let point3 = Point3D::new(2.0, 2.0, 2.0);

        assert!(octree.contains(&point1));
        // Because the bounding box contains the min surface but does not contain the max surface.
        assert!(!octree.contains(&point2));
        assert!(!octree.contains(&point3));
    }

    #[test]
    /// Should find all points within the given query area.
    fn test_octree_query() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let point3 = Point3D::new(4.0, 4.0, 4.0);
        let points = vec![point1.clone(), point2];
        let mut octree = Octree::new(points.iter().collect());
        let point4 = Point3D::new(5.0, 10.0, 5.0);
        octree.insert(&point3);
        octree.insert(&point4);

        let points_for_query = vec![point1.clone(), point4.clone()];
        let bounding_box = BoundingBox::new(points_for_query.iter().collect());

        assert_eq!(
            octree.query(&bounding_box),
            HashSet::from([&point1, &point3])
        );
    }

    #[test]
    /// Should identify if a bounding boxes overlaps / intersects with an octree.
    fn test_octree_overlap() {
        let point1 = Point3D::new(0.0, 0.0, 0.0);
        let point2 = Point3D::new(10.0, 10.0, 10.0);
        let points = vec![point1, point2];
        let octree = Octree::new(points.iter().collect());

        let point3 = Point3D::new(1.0, 1.0, 1.0);
        let point4 = Point3D::new(11.0, 11.0, 11.0);
        let bounding_box1 = BoundingBox::new(vec![point3, point4].iter().collect());

        let point5 = Point3D::new(1.0, 1.0, 1.0);
        let point6 = Point3D::new(9.0, 9.0, 9.0);
        let bounding_box2 = BoundingBox::new(vec![point5, point6].iter().collect());

        let point7 = Point3D::new(11.0, 0.0, 0.0);
        let point8 = Point3D::new(20.0, 20.0, 20.0);
        let bounding_box3 = BoundingBox::new(vec![point7, point8].iter().collect());

        assert!(octree.overlaps(&bounding_box1));
        assert!(octree.overlaps(&bounding_box2));
        assert!(!octree.overlaps(&bounding_box3));
    }
}
