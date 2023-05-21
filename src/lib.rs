pub trait Locatable {
    fn get_location(&self) -> [f32; 3];
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
