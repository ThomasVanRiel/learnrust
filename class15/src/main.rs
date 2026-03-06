use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn describe(&self) {
        println!("Area: {}", self.area());
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &impl Shape) {
    println!("Area: {}", shape.area());
}

#[derive(Clone, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color { r, g, b }
    }
}
fn main() {
    let circle = Circle { radius: 1f64 };
    circle.describe();

    let rectangle = Rectangle {
        width: 2f64,
        height: 4f64,
    };
    print_area(&rectangle);

    let c = Color::from((255, 0, 128));
    let c: Color = (255, 0, 128).into();
    let mut c2 = c.clone();
    c2.r = 0;
    println!("c: {:?}, c2: {:?}", c, c2);
}
