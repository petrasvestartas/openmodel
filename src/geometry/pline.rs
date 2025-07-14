use crate::geometry::Point;
use crate::geometry::Vector;
use crate::geometry::Frame;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pline {
    /// The collection of points.
    pub points : Point,
    /// The frame of the polyline.
    pub frame: Frame,
    /// Is polyline closed, it has duplicate point.
    pub is_closed: bool
    /// Associated data - guid and name.
    pub data: Data,
}


impl fmt::Display for Pline{
    /// Log matrix.
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Pline;
    /// let pline = Pline::default();
    /// println!("{}", pline);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Pline {{ vertices: {}, Data: {} }}", 0, self.data)
    }
}