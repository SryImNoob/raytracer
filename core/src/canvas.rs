
pub use crate::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    width: usize,
    height: usize,
    data: Box<[Color]>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            data: vec![Color::new(0.0, 0.0, 0.0); width * height].into_boxed_slice(),
        }
    }

    pub fn get(&self, x: usize, y:usize) -> Color {
        return self.data[y * self.width + x];
    }

    pub fn set(&mut self, x: usize, y:usize, c: Color) {
        self.data[y * self.width + x] = c;
    }

    pub fn save(&self, path: String) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.get(0,0), Color::new(0.0, 0.0, 0.0));
    }
}