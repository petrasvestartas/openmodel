use openmodel::geometry::Point;

fn main() {
    let mut p = Point::new(1.0, 2.0, 3.0);
    p.translate(1.0, 2.0, 3.0);
    println!("Translated Point: ({}, {}, {})", p.x, p.y, p.z);

    let p2 = p.translated(1.0, 2.0, 3.0);
    println!("New Translated Point: ({}, {}, {})", p2.x, p2.y, p2.z);
}
