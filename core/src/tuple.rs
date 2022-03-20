use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple(f32,f32,f32,f32);

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3)
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2, -self.3)

    }
}

impl Mul<f32> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Div<f32> for Tuple {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

pub fn point(x : f32, y : f32, z : f32) -> Tuple {
    return Tuple(x, y, z, 1.0);
}

pub fn vector(x : f32, y : f32, z : f32) -> Tuple {
    return Tuple(x, y, z, 0.0);
}

pub fn isVector(v:Tuple) -> bool {
    return v.3 == 0.0;
}

pub fn magnitude(v : Tuple) -> f32 {
    return f32::sqrt(v.0 * v.0 + v.1 * v.1 + v.2 * v.2 + v.3 * v.3);
}

pub fn normalize(v : Tuple) -> Tuple {
    return v / magnitude(v); 
}

pub fn dot(a:Tuple, b:Tuple) -> f32 {
    return a.0 * b.0 + a.1 * b.1 + a.2 * b.2 + a.3 * b.3;
}

pub fn cross(a:Tuple, b:Tuple) -> Tuple {
    return vector(a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        assert_eq!(magnitude(vector(1.0, 2.0, 3.0)), f32::sqrt(14.0));
    }

    #[test]
    fn test_point() {
        assert_eq!(point(4.0, -4.0, 3.0), Tuple(4.0, -4.0, 3.0, 1.0));
    }
    #[test]
    fn test_vector() {
        assert_eq!(vector(4.0, -4.0, 3.0), Tuple(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn test_add() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(point(8.0, 8.0, 8.0), p + v);
    }

    #[test]
    fn test_sub() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(vector(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn test_neg() {
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(-v, vector(-5.0, -6.0, -7.0));
    }

    #[test]
    fn test_mul() {
        let v = Tuple(5.0, 6.0, 7.0, 1.0);
        assert_eq!(v * 2.0, Tuple(10.0, 12.0, 14.0, 2.0));
    }

    #[test]
    fn test_div() {
        let v = Tuple(10.0, 12.0, 14.0, 2.0);
        assert_eq!(v / 2.0, Tuple(5.0, 6.0, 7.0, 1.0));
    }
}