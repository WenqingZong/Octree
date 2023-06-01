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
