pub enum Shape<T>
where
    T: Into<f64> + Copy,
{
    Circle(T),
    Rectangle(T, T),
    Sphere(T),
    Parallelepiped(T, T, T),
}

pub mod area {
    use crate::Shape;
    use std::f64::consts::PI;

    pub fn area<T>(s: &Shape<T>) -> f64
    where
        T: Into<f64> + Copy,
    {
        match *s {
            Shape::Circle(radius) => PI * radius.into() * radius.into(),
            Shape::Rectangle(length, width) => length.into() * width.into(),
            Shape::Sphere(radius) => 4f64 * PI * radius.into() * radius.into(),
            Shape::Parallelepiped(length, width, height) => {
                2f64 * length.into() * width.into()
                    + 2f64 * length.into() * height.into()
                    + 2f64 * width.into() * height.into()
            }
        }
    }
}
pub mod volume {
    use crate::Shape;
    use std::f64::consts::PI;

    pub fn volume<T>(s: &Shape<T>) -> f64
    where
        T: Into<f64> + Copy,
    {
        match *s {
            Shape::Circle(_) => 0f64,
            Shape::Rectangle(_, _) => 0f64,
            Shape::Sphere(radius) => {
                4f64 / 3f64 * PI * radius.into() * radius.into() * radius.into()
            }
            Shape::Parallelepiped(length, width, height) => {
                length.into() * width.into() * height.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::area::area;
    use crate::volume::volume;
    use crate::Shape;
    use std::f64::consts::PI;

    #[test]
    fn area_Circle() {
        assert_eq!(area(&Shape::Circle(PI)), PI * PI * PI);
    }

    #[test]
    fn area_Rectangle() {
        assert_eq!(area(&Shape::Rectangle(10.0, 20.0)), 200f64);
    }

    #[test]
    fn area_Sphere() {
        assert_eq!(area(&Shape::Sphere(PI)), PI * PI * PI * 4f64);
    }

    #[test]
    fn area_Parallelepiped() {
        assert_eq!(area(&Shape::Parallelepiped(10, 20, 30)), 2200f64);
    }

    #[test]
    fn volume_Circle() {
        assert_eq!(volume(&Shape::Circle(PI)), 0f64);
    }

    #[test]
    fn volume_Rectangle() {
        assert_eq!(volume(&Shape::Rectangle(10 as f32, 20 as f32)), 0f64);
    }

    #[test]
    fn volume_Sphere() {
        assert_eq!(volume(&Shape::Sphere(PI)), PI * PI * PI * PI * 4f64 / 3f64);
    }

    #[test]
    fn volume_Parallelepiped() {
        assert_eq!(volume(&Shape::Parallelepiped(10, 20, 30)), 6000f64);
    }
}
