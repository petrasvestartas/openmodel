use crate::geometry::Point;
use crate::geometry::Vector;
use crate::geometry::Plane;
use crate::common::{JsonSerializable, FromJsonData};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pline {
    /// The collection of points.
    pub points: Vec<Point>,

    /// The plane of the polyline.
    pub plane: Plane,

    /// Associated data - guid and name.
    pub data: Data,
}

impl Pline {
    /// Creates a new `Pline` with default `Data`.
    ///
    /// # Arguments
    ///
    /// * `points` - The collection of points.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// use openmodel::geometry::Plane;
    /// use openmodel::geometry::Pline;
    /// let points = vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)];
    /// let Pline = Pline::new(points);
    /// ```
    ///
    pub fn new(points: Vec<Point>) -> Self {

        // Delegate plane computation to Plane::plane_from_points
        let plane = Plane::plane_from_points(&points);

        Self {
            points,
            plane,
            data: Data::default(),
        }
    }
}


impl AddAssign<&Vector> for Pline {
    /// Adds the coordinates of another point to this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Pline, Point, Vector};
    /// let mut c = Pline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// c += &v;
    /// assert_eq!(c.points[0].x, 5.0);
    /// assert_eq!(c.points[0].y, 7.0);
    /// assert_eq!(c.points[0].z, 9.0);
    /// assert_eq!(c.points[1].x, 8.0);
    /// assert_eq!(c.points[1].y, 10.0);
    /// assert_eq!(c.points[1].z, 12.0);
    /// ```
    fn add_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            p.x += other.x;
            p.y += other.y;
            p.z += other.z;
        }
    }
}

impl Add<&Vector> for Pline {
    type Output = Pline;

    /// Adds the coordinates of a vector to this point and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Pline, Point, Vector};
    /// let c = Pline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// let c2 = c + &v;
    /// assert_eq!(c2.points[0].x, 5.0);
    /// assert_eq!(c2.points[0].y, 7.0);
    /// assert_eq!(c2.points[0].z, 9.0);
    /// ```
    fn add(self, other: &Vector) -> Pline {
        let mut c = self.clone();
        for p in &mut c.points {
            p.x += other.x;
            p.y += other.y;
            p.z += other.z;
        }
        return c;
    }
}



impl SubAssign <&Vector> for Pline {
    /// Adds the coordinates of another point to this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Pline, Point, Vector};
    /// let mut c = Pline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// c -= &v;
    /// assert_eq!(c.points[0].x, -3.0);
    /// assert_eq!(c.points[0].y, -3.0);
    /// assert_eq!(c.points[0].z, -3.0);
    /// assert_eq!(c.points[1].x, 0.0);
    /// assert_eq!(c.points[1].y, 0.0);
    /// assert_eq!(c.points[1].z, 0.0);
    /// ```
    fn sub_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            p.x -= other.x;
            p.y -= other.y;
            p.z -= other.z;
        }
    }
}

impl Sub<&Vector> for Pline {
    type Output = Pline;

    /// Adds the coordinates of a vector to this point and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Pline, Point, Vector};
    /// let c = Pline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// let c2 = c - &v;
    /// assert_eq!(c2.points[0].x, -3.0);
    /// assert_eq!(c2.points[0].y, -3.0);
    /// assert_eq!(c2.points[0].z, -3.0);
    /// assert_eq!(c2.points[1].x, 0.0);
    /// assert_eq!(c2.points[1].y, 0.0);
    /// assert_eq!(c2.points[1].z, 0.0);
    /// ```
    fn sub(self, other: &Vector) -> Pline {
        let mut c = self.clone();
        for p in &mut c.points {
            p.x -= other.x;
            p.y -= other.y;
            p.z -= other.z;
        }
        return c;
    }
}

impl fmt::Display for Pline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pline {{ points: {}, data: {} }}",
            self.points.len(),
            self.data
        )
    }
}

// JSON serialization support
impl JsonSerializable for Pline {
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

impl FromJsonData for Pline {
    fn from_json_data(data: &serde_json::Value) -> Option<Self> {
        serde_json::from_value(data.clone()).ok()
    }
}
