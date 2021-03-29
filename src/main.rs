fn main() {
    let square = Shape::Square(5_u32);
    let circle = Shape::Circle(6_f64,3.1415926_f64);
    let right_triangle = Shape::RightTriangle(2_u8,3_u8);

    println!("Square's area is {}", square.get_area());
    println!("Right Triangle's area is {}", right_triangle.get_area());
    println!("circle's area is {}", circle.get_area());
}

enum Shape<T> {
    Circle(T,T),
    Square(T),
    RightTriangle(T, T),
}

trait Area<T> {
    fn get_area(&self) -> T;
}

impl<T> Area<T> for Shape<T>
where
    T: std::ops::Mul<Output = T> + Clone + Copy,
{
    fn get_area(&self) -> T {
        match *self {
            Shape::Circle(a,pi) => a * a * pi,
            Shape::Square(a) => a * a,
            Shape::RightTriangle(a, b) => a * b,
        }
    }
}
