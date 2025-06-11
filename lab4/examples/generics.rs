use num::cast::ToPrimitive;
use num::Zero;
struct Pair<T> {
    x: T,
    y: T
}

impl<T: PartialOrd + Copy> Pair<T> {
    fn bigger(&self) -> T {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}

fn max<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }

    let mut max_value = list[0];
    for &item in list.iter() {
        if item > max_value {
            max_value = item;
        }
    }

    Some(max_value)
}

fn mean<T: num::Num + Copy + ToPrimitive>(list: &[T]) -> Option<f64> {
    if list.is_empty() {
        return None;
    }
    let mut sum = T::zero();
    for &item in list.iter() {
        sum = sum + item;
    }
    let f64_sum = sum.to_f64()?;
    let length = list.len();
    Some(f64_sum / length as f64)
}

fn main() {
    let pi = Pair{x : 5, y : 3};
    println!("{}", pi.bigger());
    println!("{:?}", mean(&[1,2, 3, 4, 10]))
}