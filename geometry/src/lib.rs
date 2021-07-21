pub enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Sphere(f32),
    Parallelepiped(f32, f32, f32),
}

pub mod area {
    use crate::Shape;
    use std::f32::consts::PI;

    pub fn area(s: &Shape) -> f32 {
        match s {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Rectangle(length, width) => length * width,
            Shape::Sphere(radius) => 4f32 * PI * radius * radius,
            Shape::Parallelepiped(length, width, height) => {
                2f32 * length * width + 2f32 * length * height + 2f32 * width * height
            }
        }
    }
}
pub mod volume {
    use crate::Shape;
    use std::f32::consts::PI;

    pub fn volume(s: &Shape) -> f32 {
        match s {
            Shape::Circle(_) => 0f32,
            Shape::Rectangle(_, _) => 0f32,
            Shape::Sphere(radius) => 4f32 / 3f32 * PI * radius * radius * radius,
            Shape::Parallelepiped(length, width, height) => length * width * height,
        }
    }
}
