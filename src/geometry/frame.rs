use crate::geometry::Point;
use crate::geometry::Vector;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    /// The origin point.
    pub origin: Point,
    /// The x-axis.
    pub xaxis: Vector,
    /// The x-axis.
    pub yaxis: Vector,
    /// The x-axis.
    pub zaxis: Vector,
    /// The first plane equation number.
    pub a : f64,
    /// The second plane equation number.
    pub b : f64,
    /// The third plane equation number.
    pub c : f64,
    /// The fourth plane equation number.
    pub d : f64,
    /// Associated data - guid and name.
    pub data: Data,
}

impl fmt::Display for Frame{
    /// Log color.
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Frame;
    /// let color = Frame::default();
    /// println!("{}", color);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Frame {{ origin: {}, xaxis {}, yaxis: {}, zaxis: {}, Data: {} }}", self.origin, self.xaxis, self.yaxis, self.zaxis, self.data)
    }
}