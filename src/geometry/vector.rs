/// A 3D vector.
pub struct Vector {
    /// The x component of the vector.
    pub x: f64,
    /// The y component of the vector.
    pub y: f64,
    /// The z component of the vector.
    pub z: f64,
}

#[allow(dead_code)]
impl Vector {
    /// Creates a new `Vector`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x component.
    /// * `y` - The y component.
    /// * `z` - The z component.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    /// Computes the length (magnitude) of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::new(1.0, 2.0, 2.0);
    /// assert_eq!(v.length(), 3.0);
    /// ```
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Default for Vector {
    /// Creates a default `Vector` with all components set to 0.0.
    fn default() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
