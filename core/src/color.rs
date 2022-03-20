use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(f32, f32, f32);

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(r, g, b)
    }

    pub fn clamp(self) -> Color {
        Color(
            clamp(self.0, 0.0, 1.0),
            clamp(self.1, 0.0, 1.0),
            clamp(self.2, 0.0, 1.0),
        )
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Color) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (clamp(self.0, 0.0, 1.0) * 255.0).round() as i64,
            (clamp(self.1, 0.0, 1.0) * 255.0).round() as i64,
            (clamp(self.2, 0.0, 1.0) * 255.0).round() as i64
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(Color(-1.0, 2.0, 0.5).to_string(), "0 255 128");
    }
}
