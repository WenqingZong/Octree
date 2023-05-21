use octree::Locatable;

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

fn main() {
    let point = Point3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    dbg!(point.get_location());
}
