use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    /// Red value from 0 to 255.
    pub r : usize,
    /// Green value from 0 to 255.
    pub g: usize,
    /// Blue value from 0 to 255.
    pub b: usize,
    /// Alpha value from 0 to 100.
    pub a: usize,
    /// Associated data - guid and name.
    pub data: Data,
}

impl Color {
    /// Creates a new `Color` with default `Data`.
    ///
    /// # Arguments
    ///
    /// * `r` - The red value 0-255.
    /// * `g` - The red value 0-255.
    /// * `b` - The red value 0-255.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Color;
    /// let c = Color::new(0, 100, 200);
    /// assert_eq!(c.r, 0);
    /// assert_eq!(c.g, 100);
    /// assert_eq!(c.b, 200);
    /// ```
    pub fn new(r: usize, g: usize, b: usize, a: usize) -> Self {
        Color {
            r,
            g,
            b,
            a,
            data: Data::with_name("Color"),
        }
    }
}


impl Default for Color {
    /// Creates a default `Color` with all values set to 0.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Color;
    /// let c = Color::default();
    /// assert_eq!(c.r, 0);
    /// assert_eq!(c.g, 0);
    /// assert_eq!(c.b, 0);
    /// ```
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
            data: Data::with_name("Color"),
        }
    }
}

impl Index<usize> for Color {
    type Output = usize;

    /// Provides read-only access to the coordinates of the color using the `[]` operator.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate (0 for r, 1 for g, 2 for b, 3 for a).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Color;
    /// let c = Color::new(100, 200, 255, 0);
    /// assert_eq!(line[0], 100);
    /// assert_eq!(line[1], 200);
    /// assert_eq!(line[2], 255);
    /// assert_eq!(line[3], 0);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Color {
    /// Provides mutable access to the coordinates of the line using the `[]` operator.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate (0 for x0, 1 for y0, 2 for z0, 3 for x1, 4 for y1, 5 for z1).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Color;
    /// let mut c = Color::new(100, 200, 255, 0);
    /// line[0] = 50;
    /// line[1] = 100;
    /// line[2] = 150;
    /// line[3] = 100;
    /// assert_eq!(line[0], 50);
    /// assert_eq!(line[1], 100);
    /// assert_eq!(line[2], 150);
    /// assert_eq!(line[3], 100);
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("Index out of bounds"),
        }
    }

}



impl fmt::Display for Color{
    /// Log color.
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Color;
    /// let color = Cloud::default();
    /// println!("{}", color);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Cloud {{ r: {}, g {}, b: {}, a: {}, Data: {} }}", self.r, self.g, self.b, self.a, self.data)
    }
}