use crate::vector::Vector3;

// Generate a random float
pub fn random_float() -> f32 {
    rand::random::<f32>()
}

// Generate a random point in 3D space, discard if outside of the unit sphere
pub fn random_in_unit_sphere() -> Vector3 {
    let mut p;

    loop {
        p = Vector3::new(random_float(), random_float(), random_float()) * 2.0 - Vector3::new(1.0, 1.0, 1.0);

        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}
