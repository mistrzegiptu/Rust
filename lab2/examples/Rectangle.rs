trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
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
}


fn main(){
    let mut r = Rectangle{x: 5.0, y: 4.0};
    r.scale(2.0);
    //Rectangle::scale(&mut r, 2.0);
    println!("Area of r is {}", r.area());
}