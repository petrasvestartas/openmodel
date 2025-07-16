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
    pub points: Vec<Point>,

    /// The collection of normals.
    pub normals: Vec<Vector>,

    /// The collection of colors.
    pub colors: Vec<Color>,

    /// The transformation matrix.
    pub xform: XForm,

    /// Associated data - guid and name.
    pub data: Data,
}



use std::fmt;

impl fmt::Display for Cloud {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cloud {{ points: {}, normals: {}, colors: {}, data: {} }}",
            self.points.len(),
            self.normals.len(),
            self.colors.len(),
            self.data
        )
    }
}
