pub use crate::color::Color;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::Write;

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

    pub fn get(&self, x: usize, y: usize) -> Color {
        return self.data[y * self.width + x];
    }

    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        self.data[y * self.width + x] = c;
    }

    pub fn save(&self, path: String) -> Result<(), std::io::Error> {
        let mut output = File::create(&path)?;
        write!(output, "{}", &self.to_string())?;
        Ok(())
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut str = String::new();
        str.push_str("P3\n");
        str.push_str(&format!("{} {}\n", self.width, self.height));
        str.push_str("255\n");
        for i in 0..(self.width * self.height) {
            let color = &self.data[i];
            str.push_str(&color.to_string());
            if i % self.width == self.width - 1 {
                str.push_str("\n")
            } else {
                str.push_str(" ")
            }
        }
        // for color in &self.data[0..self.data.len()] {
        //     str.push_str(&color.to_string());
        //     str.push_str(" ");
        // }
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.get(0, 0), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_fmt() {
        let mut c = Canvas::new(2, 2);
        c.set(0, 0, Color::new(1.0, 0.0, 0.0));
        c.set(0, 1, Color::new(0.0, 0.5, 0.0));
        c.set(1, 1, Color::new(0.0, 0.0, 1.0));
        assert_eq!(
            c.to_string(),
            "P3\n2 2\n255\n255 0 0 0 0 0\n0 128 0 0 0 255\n"
        );
    }
}
