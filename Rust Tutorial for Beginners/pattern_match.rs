enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Rectangle(height, width) => height * width,
    }
}

fn main() {
    let _circle = Shape::Circle(5.0);
    let _square = Shape::Square(5.0);
    let rectangle = Shape::Rectangle(5.0, 6.0);
    println!("Area of rectangle is {:?}", calculate_area(rectangle));
}