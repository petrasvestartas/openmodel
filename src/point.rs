pub struct Point {
    /// The x coordinate of the point.
    pub x: f64,
    /// The y coordinate of the point.
    pub y: f64,
    /// The z coordinate of the point.
    pub z: f64,
}

#[allow(dead_code)]
impl Point {
    /// Creates a new `Vector`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate.
    /// * `y` - The y coordinate.
    /// * `z` - The z coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::point::Point;
    /// let v = Point::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    /// Computes the distance between two points.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::point::Point;
    /// let p1 = Point::new(1.0, 2.0, 2.0);
    /// let p2 = Point::new(4.0, 6.0, 6.0);
    /// ```
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    /// Translates the point by the given amounts.
    ///
    /// # Arguments
    ///
    /// * `dx` - The offset in the x direction.
    /// * `dy` - The offset in the y direction (default is 0.0).
    /// * `dz` - The offset in the z direction (default is 0.0).
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::point::Point;
    /// let mut p = Point::new(1.0, 2.0, 2.0);
    /// p.translate(1.0, 2.0, 3.0);
    /// assert_eq!(p.x, 2.0);
    /// assert_eq!(p.y, 4.0);
    /// assert_eq!(p.z, 5.0);
    /// ```
    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }


    /// Returns a new point translated by the given amounts.
    ///
    /// # Arguments
    ///
    /// * `dx` - The offset in the x direction.
    /// * `dy` - The offset in the y direction (default is 0.0).
    /// * `dz` - The offset in the z direction (default is 0.0).
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::point::Point;
    /// let p = Point::new(1.0, 2.0, 2.0);
    /// let p2 = p.translated(1.0, 2.0, 3.0);
    /// assert_eq!(p2.x, 2.0);
    /// assert_eq!(p2.y, 4.0);
    /// assert_eq!(p2.z, 5.0);
    /// ```
    pub fn translated(&self, dx: f64, dy: f64, dz: f64) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
