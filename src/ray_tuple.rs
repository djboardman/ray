use std::ops::{Add, Sub, Neg, Mul, Div};
use std::cmp;

const EPSILON: f32 = 0.00001;

#[derive(Clone, Debug)]
pub struct RayTuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl RayTuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> RayTuple {
        RayTuple { x: x, y: y, z: z, w: w }
    } 

    pub fn point(x: f32, y: f32, z: f32) -> RayTuple {
        RayTuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> RayTuple {
        RayTuple::new(x, y, z, 0.0)
    }

    pub fn zero_vector() -> RayTuple {
        RayTuple::vector(0.0, 0.0, 0.0)
    }

    pub fn color(r: f32, g: f32, b: f32) -> RayTuple {
        RayTuple::new(r, g, b, 0.0)
    }
  
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> RayTuple {
        let mag = self.magnitude();
        RayTuple::new(self.x/mag, self.y/mag, self.z/mag, self.w/mag)
    }

    pub fn dot(&self, other: &RayTuple) -> f32 {
        self.x * &other.x + self.y * &other.y + self.z * &other.z + self.w * &other.w
    }

    pub fn cross(&self, other: &RayTuple) -> RayTuple {
        RayTuple::vector(self.y * &other.z - self.z * &other.y,
                         self.z * &other.x - self.x * &other.z,
                         self.x * &other.y - self.y * &other.x)   
    }

    pub fn scaled(&self, min: isize, max: isize) -> String {
        let x = RayTuple::scale(self.x, min, max);
        let y = RayTuple::scale(self.y, min, max);
        let z = RayTuple::scale(self.z, min, max);
        let s = format!("{} {} {}", x, y, z);
        return s;
    }

    fn scale(val: f32, min: isize, max: isize) -> isize {
        cmp::max(0, cmp::min(((max - min) as f32 * val).round() as isize, max))
    }

}

impl PartialEq for RayTuple {
    fn eq(&self, other: &RayTuple) -> bool {
        let diff = self.clone() - other.clone();
        diff.x.abs() < EPSILON && diff.y.abs() < EPSILON && diff.z.abs() < EPSILON && diff.w.abs() < EPSILON
    }
}

impl Eq for RayTuple {}
impl Add for RayTuple {
    type Output = RayTuple;

    fn add(self, rhs: RayTuple) -> RayTuple {
        RayTuple::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z,self.w + rhs.w)
    }
}

impl Sub for RayTuple {
    type Output = RayTuple;

    fn sub(self, rhs: RayTuple) -> RayTuple {
        RayTuple::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl Neg for RayTuple {
    type Output = RayTuple;

    fn neg(self) -> RayTuple {
        RayTuple::zero_vector() - self
    }
}

impl Mul for RayTuple {
    type Output = RayTuple;

    fn mul(self, rhs: RayTuple) -> RayTuple {
        RayTuple::color(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for RayTuple {
    type Output = RayTuple;

    fn mul(self, rhs: f32) -> RayTuple {
        RayTuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f32> for RayTuple {
    type Output = RayTuple;

    fn div(self, rhs: f32) -> RayTuple {
        RayTuple::new(self.x / rhs, self.y / rhs, self.y / rhs, self.w / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_is_a_point() {
        let a = RayTuple::point(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn a_tuple_is_a_vector() {
        let a = RayTuple::vector(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = RayTuple::point(3.0, -2.0, 5.0);
        let a2 = RayTuple::vector(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, RayTuple::point(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_points() {
        let a1 = RayTuple::point(3.0, 2.0, 1.0);
        let a2 = RayTuple::point(5.0, 6.0, 7.0);
        assert_eq!(a1 - a2, RayTuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = RayTuple::vector(3.0, 2.0, 1.0);
        let v2 = RayTuple::vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, RayTuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = RayTuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert_eq!(-a, RayTuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 });
    }

    #[test]
    fn multiplying_by_a_scalar() {
        let a = RayTuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert_eq!(a * 3.5, RayTuple::new(3.5, -7.0, 10.5, -14.0));
    }
    
    #[test]
    fn computing_magnitude_1() {
        let v = RayTuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_2() {
        let v = RayTuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_3() {
        let v = RayTuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_4() {
        let v = RayTuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0f32).sqrt());
    }

    #[test]
    fn computing_magnitude_5() {
        let v = RayTuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14.0f32).sqrt());
    }
    
    #[test]
    fn normalizing_vector_1() {
        let v = RayTuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), RayTuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_2() {
        let v = RayTuple::vector(1.0, 2.0, 3.0);
        let x = 1.0/(14f32).sqrt();
        let y = 2.0/(14f32).sqrt();
        let z = 3.0/(14f32).sqrt();
        assert_eq!(v.normalize(), RayTuple::vector(x, y, z));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = RayTuple::vector(1.0, 2.0, 3.0);
        assert!((v.normalize().magnitude() - 1.0f32).abs() < EPSILON);
    }

    #[test]
    fn dot_product() {
        let a = RayTuple::vector(1.0, 2.0, 3.0);
        let b = RayTuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
        assert_eq!(b.dot(&a), 20.0);
    }
    
    #[test]
    fn cross_product() {
        let a = RayTuple::vector(1.0, 2.0, 3.0);
        let b = RayTuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), RayTuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), RayTuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn add_colors() {
        let c1 = RayTuple::color(0.9, 0.6, 0.75);
        let c2 = RayTuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, RayTuple::color(1.6, 0.7, 1.0));
    }

    #[test]
    fn sub_colors() {
        let c1 = RayTuple::color(0.9, 0.6, 0.75);
        let c2 = RayTuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, RayTuple::color(0.2, 0.5, 0.5));
    }

    #[test]
    fn scalar_mult_colors() {
        let c1 = RayTuple::color(0.2, 0.3, 0.4);
        assert_eq!(c1*2.0, RayTuple::color(0.4, 0.6, 0.8));
    }

    #[test]
    fn mult_colors() {
        let c1 = RayTuple::color(1.0, 0.2, 0.4);
        let c2 = RayTuple::color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, RayTuple::color(0.9, 0.2, 0.04));
    }
}
