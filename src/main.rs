use openmodel::geometry::Point;
use openmodel::geometry::Vector;
use openmodel::geometry::Line;

fn main() {
    let mut p = Point::new(1.0, 2.0, 3.0);
    p += &Vector::new(1.0, 2.0, 3.0);
    println!("{}", p);


    let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    line*=10.0;
    println!("{}", line);
}