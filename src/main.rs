use geometry::area::area;
use geometry::volume::volume;
use geometry::Shape;
use std::env;

fn main() {
    let mut args = env::args();
    let _path = args.next();
    let shape_str = match args.next() {
        None => {
            println!("Usage: Shape x1 x2 x3");
            println!("\tCircle radius");
            println!("\tRectangle length width");
            println!("\tSphere radius");
            println!("\tParallelepiped length width height");
            return;
        }
        Some(str) => str,
    };
    let shape_parameters: Vec<f32> = args.filter_map(|e| e.parse().ok()).collect();
    let shape = match &shape_str[..] {
        "Circle" => {
            if shape_parameters.len() == 1 {
                Shape::Circle(shape_parameters[0])
            } else {
                println!("Circle must only have radius");
                return;
            }
        }
        "Rectangle" => {
            if shape_parameters.len() == 2 {
                Shape::Rectangle(shape_parameters[0], shape_parameters[1])
            } else {
                println!("Rectangle must only have length and width");
                return;
            }
        }
        "Sphere" => {
            if shape_parameters.len() == 1 {
                Shape::Sphere(shape_parameters[0])
            } else {
                println!("Sphere must only have radius");
                return;
            }
        }
        "Parallelepiped" => {
            if shape_parameters.len() == 3 {
                Shape::Parallelepiped(
                    shape_parameters[0],
                    shape_parameters[1],
                    shape_parameters[2],
                )
            } else {
                println!("Parallelepiped must only have length, width and height");
                return;
            }
        }
        _ => {
            println!("Shape must be Circle, Rectangle, Sphere or Parallelepiped");
            return;
        }
    };
    println!("Area = {:?}; Volume = {:?}", area(&shape), volume(&shape));
}
