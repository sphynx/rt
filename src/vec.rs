use std::ops::*;

/// Type of vector elements (currently it's f64).
pub type Elem = f64;

/// 3D vector.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(pub Elem, pub Elem, pub Elem);

impl Vec3 {
    pub fn x(&self) -> Elem {
        self.0
    }

    pub fn y(&self) -> Elem {
        self.1
    }

    pub fn z(&self) -> Elem {
        self.2
    }

    pub fn r(&self) -> Elem {
        self.0
    }

    pub fn g(&self) -> Elem {
        self.1
    }

    pub fn b(&self) -> Elem {
        self.2
    }

    pub fn length_squared(&self) -> Elem {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> Elem {
        self.length_squared().sqrt()
    }

    pub fn make_unit_vector(&mut self) {
        let k = self.length();
        self.0 /= k;
        self.1 /= k;
        self.2 /= k;
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn dot(&self, v: &Vec3) -> Elem {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    pub fn dot2(v: Vec3, w: Vec3) -> Elem {
        v.x() * w.x() + v.y() * w.y() + v.z() * w.z()
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        let x = self.y() * v.z() - self.z() * v.y();
        let y = -(self.x() * v.z() - self.z() * v.x());
        let z = self.x() * v.y() - self.y() * v.x();
        Vec3(x, y, z)
    }

    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Vec3 {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn one_x() -> Vec3 {
        Vec3(1.0, 0.0, 0.0)
    }

    pub fn one_y() -> Vec3 {
        Vec3(0.0, 1.0, 0.0)
    }

    pub fn one_z() -> Vec3 {
        Vec3(0.0, 0.0, 1.0)
    }
}

impl Vec3 {
    pub fn sqrt_coords(&mut self)  {
        self.0 = self.0.sqrt();
        self.1 = self.1.sqrt();
        self.2 = self.2.sqrt();
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Mul<Elem> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Elem) -> Vec3 {
        Vec3(other * self.0, other * self.1, other * self.2)
    }
}

impl Mul<Vec3> for Elem {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl MulAssign<Elem> for Vec3 {
    fn mul_assign(&mut self, other: Elem) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<Elem> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Elem) -> Vec3 {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl DivAssign<Elem> for Vec3 {
    fn div_assign(&mut self, other: Elem) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let x = Vec3(1., 2., 3.);
        let y = x + x;
        assert_eq!(y.0, 2.);
        assert_eq!(y.1, 4.);
        assert_eq!(y.2, 6.);
    }

    #[test]
    fn test_2() {
        let x = Vec3(1., 2., 3.);
        let y = x * 2.0;
        assert_eq!(y.0, 2.);
        assert_eq!(y.1, 4.);
        assert_eq!(y.2, 6.);
    }

    #[test]
    fn test_3() {
        let x = Vec3(3., 4., 0.);
        assert_eq!(x.length(), 5.);
    }

    #[test]
    fn test_4() {
        let mut x = Vec3(3., 0., 0.);
        x.make_unit_vector();
        assert_eq!(x.length(), 1.);
    }

}
