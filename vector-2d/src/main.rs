mod test;

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Vec2D{
    x: f32,
    y: f32,
}

impl Vec2D {
    fn scalar_product(&self, other: &Vec2D) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn mul_by_scalar(&self, scalar: f32) -> Vec2D {
        Vec2D {x: self.x * scalar, y: self.y * scalar}
    }

    fn to_unit_vec(&self) -> Vec2D {
        let vec_len = self.vec_len();
        if vec_len == 0.0 {
            panic!("attempt to divide by zero")
        }
        Vec2D {x: self.x / vec_len,
               y: self.y / vec_len}
    }

    fn vec_len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for &Vec2D{
    type Output = Vec2D;

    fn add(self, other: &Vec2D) -> Vec2D {
        Vec2D {x: self.x + other.x,
               y: self.y + other.y}
    }
}

impl Sub for &Vec2D {
    type Output = Vec2D;

    fn sub(self, other: &Vec2D) -> Vec2D {
        Vec2D {x: self.x - other.x,
               y: self.y - other.y}
    }
}

impl PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2D[x={},y={}]", self.x, self.y)?;
        Ok(())
    }
}

fn main() {
    let v1 = Vec2D{x: 10.0, y: 10.0};
    let v2 = Vec2D{x: 5.0, y: 5.0};

    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("v1 scal_prod v2: {}", v1.scalar_product(&v2));
    println!("v1 * 10: {}", v1.mul_by_scalar(10.0));
    println!("v1+v2: {}", &v1+&v2);
    println!("v1-v2: {}", &v1-&v2);
    println!("v1 to unit_vec: {}", v1.to_unit_vec());
}
