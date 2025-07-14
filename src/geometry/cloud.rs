use crate::geometry::Point;
use crate::geometry::Vector;
use crate::geometry::Frame;
use crate::geometry::Matrix;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cloud {
    /// The collection of points.
    pub points : Point,
    /// The collection of vectors.
    pub normals: Vector,
    /// The collection of colors.
    pub colors: Vector,
    /// The transformation matrix.
    pub matrix: Matrix,
    /// Associated data - guid and name.
    pub data: Data,
}


impl fmt::Display for Cloud{
    /// Log cloud.
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Cloud;
    /// let cloud = Cloud::default();
    /// println!("{}", cloud);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Cloud {{ points: {}, normals {}, colors: {}, Data: {} }}", 0, 0, 0, self.data)
    }
}