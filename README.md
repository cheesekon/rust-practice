# Rust practice

## Trait

```bash
fn main() {
    let yellow: TrafficLight = TrafficLight::Yellow;
    let red: TrafficLight = TrafficLight::Red;
    let green: TrafficLight = TrafficLight::Green;

    println!("Yellow is {}", yellow.time());
    println!("Red is {}", red.time());
    println!("Green is {}", green.time());
}

trait TrafficLightTrait {
    fn time(&self) -> u8;
}

enum TrafficLight {
    Red,
    Green,
    Yellow
}

impl TrafficLightTrait for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 15,
            TrafficLight::Yellow => 5
        }
    }
}
```

## Option & Overflow

```bash
fn main() {
    let arr1: [u32; 5] = [999999999, 999999999, 999999999, 999999999, 999999999];
    let arr2: [u32; 5] = [1, 2, 3, 4, 5];

    println!("The arr1's summation result is {:?}", summation(&arr1));
    println!("The arr2's summation result is {:?}", summation(&arr2));
}
fn summation(arr: &[u32]) -> Option<u32> {
    let mut res: Option<u32> = Some(0);

    for &i in arr {
        res = res.unwrap().checked_add(i);
    }
    
    res
}

```

## Generic

```bash
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

```