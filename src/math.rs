use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    #[inline]
    pub fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    pub fn normalized(self) -> Vector3 {
        let mag = self.magnitude();
        Vector3::new(self.x / mag, self.y / mag, self.z / mag)
    }

    #[inline]
    pub fn cross(self, Vector3 { x, y, z }: Vector3) -> Vector3 {
        Vector3::new(
            self.x.mul_add(z, -self.z * y),
            self.y.mul_add(x, -self.x * z),
            self.z.mul_add(y, -self.y * x),
        )
    }
}

impl From<Vector3> for [f32; 3] {
    #[inline]
    fn from(Vector3 { x, y, z }: Vector3) -> Self {
        [x, y, z]
    }
}

impl From<mint::Vector3<f32>> for Vector3 {
    #[inline]
    fn from(mint::Vector3 { x, y, z }: mint::Vector3<f32>) -> Self {
        Vector3 { x, y, z }
    }
}

impl From<Vector3> for mint::Vector3<f32> {
    #[inline]
    fn from(Vector3 { x, y, z }: Vector3) -> Self {
        mint::Vector3 { x, y, z }
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    #[inline]
    fn sub(self, Vector3 { x, y, z }: Self) -> Self::Output {
        Vector3::new(self.x - x, self.y - y, self.z - z)
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, Vector3 { x, y, z }: Self) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}
