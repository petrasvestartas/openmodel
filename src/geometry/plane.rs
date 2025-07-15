use crate::geometry::Point;
use crate::geometry::Vector;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    /// The origin point.
    pub origin: Point,
    /// The x-axis.
    pub xaxis: Vector,
    /// The x-axis.
    pub yaxis: Vector,
    /// The x-axis.
    pub zaxis: Vector,
    /// The normal x coordinate.
    pub a : f64,
    /// The normal y coordinate.
    pub b : f64,
    /// The normal z coordinate.
    pub c : f64,
    /// The plane offset from origin.
    pub d : f64,
    /// Associated data - guid and name.
    pub data: Data,
}


impl Plane{
    /// Creates a new `Line` with default `Data`.
    ///
    /// # Arguments
    ///
    /// * `x0` - The x components of the start point.
    /// * `y0` - The y components of the start point.
    /// * `z0` - The z components of the start point.
    /// * `x1` - The x components of the end point.
    /// * `y1` - The y components of the end point.
    /// * `z1` - The z components of the end point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Line;
    /// let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// assert_eq!(line.x0, 0.0);
    /// assert_eq!(line.y0, 0.0);
    /// assert_eq!(line.z0, 0.0);
    /// assert_eq!(line.x1, 0.0);
    /// assert_eq!(line.y1, 0.0);
    /// assert_eq!(line.z1, 1.0);
    /// 
    /// ```
    pub fn new(origin: Point, xaxis: Vector, yaxis: Vector) -> Self {
        let zaxis = Vector::cross(&xaxis, &yaxis);
        let a = zaxis.x;
        let b = zaxis.y;
        let c = zaxis.z;
        let d = -a * origin.x - b * origin.y - c * origin.z;
        Plane {
            origin,
            xaxis,
            yaxis,
            zaxis,
            a,
            b,
            c,
            d,
            data: Data::with_name("Plane")
        }
    }

    /// Creates a new `Plane` with a specified name for `Data`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the `Data`.
    /// * `origin` - The origin point.
    /// * `xaxis` - The x-axis.
    /// * `yaxis` - The y-axis.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Plane, Point, Vector};
    /// let plane = Plane::with_name("MyPlane".to_string(), Point::new(0.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    /// assert_eq!(plane.origin.x, 0.0);
    /// assert_eq!(plane.origin.y, 0.0);
    /// assert_eq!(plane.origin.z, 0.0);
    /// assert_eq!(plane.xaxis.x, 1.0);
    /// assert_eq!(plane.xaxis.y, 0.0);
    /// assert_eq!(plane.xaxis.z, 0.0);
    /// assert_eq!(plane.yaxis.x, 0.0);
    /// assert_eq!(plane.yaxis.y, 1.0);
    /// assert_eq!(plane.yaxis.z, 0.0);
    /// assert_eq!(plane.zaxis.x, 0.0);
    /// assert_eq!(plane.zaxis.y, 0.0);
    /// assert_eq!(plane.zaxis.z, 1.0);
    /// assert_eq!(plane.a, 0.0);
    /// assert_eq!(plane.b, 0.0);
    /// assert_eq!(plane.c, 1.0);
    /// assert_eq!(plane.d, 0.0);
    /// ```
    pub fn with_name(name: String, origin: Point, xaxis: Vector, yaxis: Vector) -> Self {
        let zaxis = Vector::cross(&xaxis, &yaxis);
        let a = zaxis.x;
        let b = zaxis.y;
        let c = zaxis.z;
        let d = -a * origin.x - b * origin.y - c * origin.z;
        Plane {
            origin,
            xaxis,
            yaxis,
            zaxis,
            a,
            b,
            c,
            d,
            data: Data::with_name(&name)
        }
    }

    


}

impl Default for Plane {
    /// Creates a zero length `Plane`.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Plane;
    /// let plane = Plane::default();
    /// assert_eq!(plane.origin.x, 0.0);
    /// assert_eq!(plane.origin.y, 0.0);
    /// assert_eq!(plane.origin.z, 0.0);
    /// assert_eq!(plane.xaxis.x, 1.0);
    /// assert_eq!(plane.xaxis.y, 0.0);
    /// assert_eq!(plane.xaxis.z, 0.0);
    /// assert_eq!(plane.yaxis.x, 0.0);
    /// assert_eq!(plane.yaxis.y, 1.0);
    /// assert_eq!(plane.yaxis.z, 0.0);
    /// assert_eq!(plane.zaxis.x, 0.0);
    /// assert_eq!(plane.zaxis.y, 0.0);
    /// assert_eq!(plane.zaxis.z, 1.0);
    /// assert_eq!(plane.a, 0.0);
    /// assert_eq!(plane.b, 0.0);
    /// assert_eq!(plane.c, 1.0);
    /// assert_eq!(plane.d, 0.0);
    /// ```
    fn default() -> Self {
        Plane {
            origin: Point::new(0.0, 0.0, 0.0),
            xaxis: Vector::new(1.0, 0.0, 0.0),
            yaxis: Vector::new(0.0, 1.0, 0.0),
            zaxis: Vector::new(0.0, 0.0, 1.0),
            a: 0.0,
            b: 0.0,
            c: 1.0,
            d: 0.0,
            data: Data::with_name("Plane"),
        }
    }
}




impl Add<&Vector> for Plane {
    type Output = Plane;

    /// Adds the coordinates of a vector to this plane and returns a new plane.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Plane, Vector};
    /// let plane0 = Plane::default();
    /// let v = Vector::new(0.0, 0.0, 1.0);
    /// let plane1 = plane0 + &v;
    /// assert_eq!(plane1.origin.x, 0.0);
    /// assert_eq!(plane1.origin.y, 0.0);
    /// assert_eq!(plane1.origin.z, 1.0);
    /// ```
    fn add(self, other: &Vector) -> Plane {
        Plane {
            origin: self.origin + other,
            xaxis: self.xaxis,
            yaxis: self.yaxis,
            zaxis: self.zaxis,
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            data: Data::with_name("Plane"),
        }
    }
}


impl AddAssign<&Vector> for Plane {
    /// Adds the coordinates of a vector to this plane.
    ///
    /// # Arguments
    ///
    /// * `vector` - traslation vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Plane;
    /// use openmodel::geometry::Vector;
    /// let mut plane = Plane::default();
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// plane += &v;
    /// assert_eq!(plane.origin.x, 1.0);
    /// assert_eq!(plane.origin.y, 1.0);
    /// assert_eq!(plane.origin.z, 1.0);
    /// ```
    fn add_assign(&mut self, vector: &Vector) {
        self.origin += vector;
    }
}


impl Sub<&Vector> for Plane {
    type Output = Plane;

    /// Subtracts the coordinates of a vector to this plane and returns a new plane.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Plane, Vector};
    /// let plane0 = Plane::default();
    /// let v = Vector::new(0.0, 0.0, 1.0);
    /// let plane1 = plane0 - &v;
    /// assert_eq!(plane1.origin.x, 0.0);
    /// assert_eq!(plane1.origin.y, 0.0);
    /// assert_eq!(plane1.origin.z, -1.0);
    /// ```
    fn sub(self, vector: &Vector) -> Plane {
        Plane {
            origin: self.origin - vector,
            xaxis: self.xaxis,
            yaxis: self.yaxis,
            zaxis: self.zaxis,
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            data: Data::with_name("Plane"),
        }
    }
}


impl SubAssign<&Vector> for Plane {
    /// Subtracts the coordinates of a vector to this plane.
    ///
    /// # Arguments
    ///
    /// * `vector` - traslation vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Plane;
    /// use openmodel::geometry::Vector;
    /// let mut plane = Plane::default();
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// plane -= &v;
    /// assert_eq!(plane.origin.x, -1.0);
    /// assert_eq!(plane.origin.y, -1.0);
    /// assert_eq!(plane.origin.z, -1.0);
    /// ```
    fn sub_assign(&mut self, vector: &Vector) {
        self.origin -= vector;
    }
}


impl fmt::Display for Plane{
    /// Log color.
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Plane;
    /// let plane = Plane::default();
    /// println!("{}", plane);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Plane {{ origin: {}, xaxis {}, yaxis: {}, zaxis: {}, Data: {} }}", self.origin, self.xaxis, self.yaxis, self.zaxis, self.data)
    }
}