use std::ops::{Index, IndexMut};

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
    /// use openmodel::geometry::Point;
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
    /// use openmodel::geometry::Point;
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
    /// use openmodel::geometry::Point;
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
    /// use openmodel::geometry::Point;
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
    /// Creates a default `Point` with all coordinates set to 0.0.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p = Point::default();
    /// assert_eq!(p.x, 0.0);
    /// assert_eq!(p.y, 0.0);
    /// assert_eq!(p.z, 0.0);
    /// ```
    fn default() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Index<usize> for Point {
    type Output = f64;

    /// Provides read-only access to the coordinates of the point using the `[]` operator.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate (0 for x, 1 for y, 2 for z).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p = Point::new(1.0, 2.0, 3.0);
    /// assert_eq!(p[0], 1.0);
    /// assert_eq!(p[1], 2.0);
    /// assert_eq!(p[2], 3.0);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Point {
    /// Provides mutable access to the coordinates of the point using the `[]` operator.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate (0 for x, 1 for y, 2 for z).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let mut p = Point::new(1.0, 2.0, 3.0);
    /// p[0] = 4.0;
    /// p[1] = 5.0;
    /// p[2] = 6.0;
    /// assert_eq!(p[0], 4.0);
    /// assert_eq!(p[1], 5.0);
    /// assert_eq!(p[2], 6.0);
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}