use std::fmt::{Display, Formatter};

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;

    fn describe(&self) {
        println!("I'm a general shape.");
    }
}

struct Rectangle{
    x: f32,
    y: f32,
}

impl Rectangle{
    fn scale(&mut self, scale: f32){
        self.x *= scale;
        self.y *= scale;
    }

    fn area(&self) -> f32 {
        self.x * self.y
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.x * self.y * 2.0
    }

    fn perimeter(&self) -> f32 {
        2f32 * (self.x + self.y)
    }

    fn describe(&self) {
        println!("I'm a rectangle.");
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle[x={},y={}]", self.x, self.y)?;
        Ok(())
    }
}

fn main(){
    let mut r = Rectangle{x: 5.0, y: 4.0};
    r.scale(2.0);
    //Rectangle::scale(&mut r, 2.0);
    //println!("Area of r as shape is {}", <Rectangle as Shape>::area(&r));
    r.describe();
    println!("Area of r is {}", r.area());
    println!("{}", r);
}