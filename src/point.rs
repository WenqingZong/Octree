/// The default point structure that this [Octree] holds.
use crate::Locatable;
#[derive(Debug, Default, PartialEq)]
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
