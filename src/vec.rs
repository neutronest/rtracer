use std::num;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};

struct Vec3 {
    element: [f32; 3]
}

impl Vec3 {
    fn x(&self) -> f32 {
        self.element[0]
    }

    fn y(&self) -> f32 {
        self.element[1]
    }

    fn z(&self) -> f32 {
        self.element[2]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Vec3 {
            element: [-1.0 * self.x(), -1.0 * self.y(), -1.0 * self.z()]
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Self {
            element: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            element: [self.x() - other.x(), self.y() - other.y(), self.z() - other.z()]
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            element: [self.x() * other.x(), self.y() * other.y(), self.z() * other.z()]
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Self) -> Self::Output {
        Self {
            element: [self.x() / other.x(), self.y() / other.y(), self.z() / other.z()]
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.element[0] *= other.x();
        self.element[1] *= other.y();
        self.element[2] *= other.z();

    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.element[0] /= other.x();
        self.element[1] /= other.y();
        self.element[2] /= other.z();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }

    #[test]
    fn test_basic_value() {
        let foo_vec : Vec3 = Vec3 { element: [1.0, 2.0, 3.0]};
        assert!((foo_vec.x() - 1.0).abs()  < 0.0001);
        assert!((foo_vec.y() - 2.0).abs()  < 0.0001);
        assert!((foo_vec.z() - 3.0).abs()  < 0.0001);
    }

    #[test]
    fn test_neg_operation() {
        let foo_vec : Vec3 = Vec3 { element: [1.0, 2.0, 3.0]};
        let neg_foo_vec: Vec3 = -foo_vec;
        assert!((neg_foo_vec.x() - (-1.0)).abs() < 0.0001 );
        assert!((neg_foo_vec.y() - (-2.0)).abs() < 0.0001 );
        assert!((neg_foo_vec.z() - (-3.0)).abs() < 0.0001 );
    }

    #[test]
    fn test_add_operation() {
        let foo_vec : Vec3 = Vec3 { element: [1.0, 2.0, 3.0]};
        let other_vec : Vec3 = Vec3 {element: [4.0, 5.0, 6.0]};

        let result_vec : Vec3 = foo_vec + other_vec;
        assert!( (result_vec.x() - 5.0).abs() < 0.0001 );
        assert!( (result_vec.y() - 7.0).abs() < 0.0001 );
        assert!( (result_vec.z() - 9.0).abs() < 0.0001 );
    }

    #[test]
    fn test_sub_operation() {
        let foo_vec : Vec3 = Vec3 { element: [1.0, 2.0, 3.0]};
        let other_vec : Vec3 = Vec3 {element: [4.0, 5.0, 6.0]};

        let result_vec : Vec3 = foo_vec - other_vec;
        assert!( (result_vec.x() - (-3.0) ).abs() < 0.0001 );
        assert!( (result_vec.y() - (-3.0) ).abs() < 0.0001 );
        assert!( (result_vec.z() - (-3.0) ).abs() < 0.0001 );
    }

    #[test]
    fn test_mul_operation() {
        let foo_vec : Vec3 = Vec3 { element: [1.0, 2.0, 3.0]};
        let other_vec : Vec3 = Vec3 {element: [4.0, 5.0, 6.0]};

        let result_vec : Vec3 = foo_vec * other_vec;
        assert!( (result_vec.x() - (4.0) ).abs() < 0.0001 );
        assert!( (result_vec.y() - (10.0) ).abs() < 0.0001 );
        assert!( (result_vec.z() - (18.0) ).abs() < 0.0001 );
    }

    #[test]
    fn test_div_operation() {
        let foo_vec : Vec3 = Vec3 { element: [1.0, 2.0, 3.0]};
        let other_vec : Vec3 = Vec3 {element: [4.0, 5.0, 6.0]};

        let result_vec : Vec3 = foo_vec / other_vec;
        assert!( (result_vec.x() - (0.25) ).abs() < 0.0001 );
        assert!( (result_vec.y() - (0.4) ).abs() < 0.0001 );
        assert!( (result_vec.z() - (0.5) ).abs() < 0.0001 );
    }

    #[test]
    fn test_mul_assign_operation() {
        let mut foo_vec : Vec3 = Vec3 { element: [1.0, 2.0, 3.0]};
        let other_vec : Vec3 = Vec3 {element: [4.0, 5.0, 6.0]};
        foo_vec *= other_vec;
        assert!( (foo_vec.x() - (4.0) ).abs() < 0.0001 );
        assert!( (foo_vec.y() - (10.0) ).abs() < 0.0001 );
        assert!( (foo_vec.z() - (18.0) ).abs() < 0.0001 );
    }

    #[test]
    fn test_div_assign_operation() {
        let mut foo_vec : Vec3 = Vec3 { element: [1.0, 2.0, 3.0]};
        let other_vec : Vec3 = Vec3 {element: [4.0, 5.0, 6.0]};
        foo_vec /= other_vec;
        assert!( (foo_vec.x() - (0.25) ).abs() < 0.0001 );
        assert!( (foo_vec.y() - (0.4) ).abs() < 0.0001 );
        assert!( (foo_vec.z() - (0.5) ).abs() < 0.0001 );
    }
}
