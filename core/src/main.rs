pub mod canvas;
pub mod color;
pub mod tuple;

fn main() {
    println!("Hello, world!");
    println!("{:?}", tuple::point(19.0, 20.0, 21.0));
    println!("{:?}", tuple::vector(19.0, 20.0, 21.0));
    let mut c = canvas::Canvas::new(10 as usize, 10 as usize);
    c.set(0, 1, color::Color::new(1.0, 1.0, 1.0));
    c.save("c.ppm".to_owned());
    println!("{:?}", c);
}
