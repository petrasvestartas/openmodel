use std::ops::{Index, IndexMut};

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


impl Index<usize> for Vector {
    type Output = f64;

    /// Provides read-only access to the coordinates of the Vector using the `[]` operator.
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
    /// use openmodel::geometry::Vector;
    /// let p = Vector::new(1.0, 2.0, 3.0);
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

impl IndexMut<usize> for Vector {
    /// Provides mutable access to the coordinates of the Vector using the `[]` operator.
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
    /// use openmodel::geometry::Vector;
    /// let mut p = Vector::new(1.0, 2.0, 3.0);
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