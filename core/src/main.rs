use core::color::Color;

pub mod tuple;
pub mod color;
pub mod canvas;

fn main() {
    println!("Hello, world!");
    println!("{:?}", tuple::point(19.0, 20.0, 21.0));
    println!("{:?}", tuple::vector(19.0, 20.0, 21.0));
    let mut c = canvas::Canvas::new(10 as usize, 10 as usize);
    c.set(0, 1, color::Color::new(1.0, 1.0, 1.0));
    println!("{:?}", c);
}
