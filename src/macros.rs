// Macro implementations for common geometry structs

/// Macro for creating a Point with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use openmodel::geometry::Point;
/// use openmodel::point;
/// use openmodel::common::Data;
/// 
/// // Default point (origin)
/// let p1 = point![];
/// assert_eq!(p1.x, 0.0);
/// assert_eq!(p1.y, 0.0);
/// assert_eq!(p1.z, 0.0);
/// 
/// // Point with coordinates
/// let p2 = point![1.0, 2.0, 3.0];
/// assert_eq!(p2.x, 1.0);
/// assert_eq!(p2.y, 2.0);
/// assert_eq!(p2.z, 3.0);
/// 
/// // Point with custom name
/// let p3 = point![4.0, 5.0, 6.0, name: "CustomPoint"];
/// assert_eq!(p3.x, 4.0);
/// assert_eq!(p3.y, 5.0);
/// assert_eq!(p3.z, 6.0);
/// assert_eq!(p3.data.name(), "CustomPoint");
/// ```
#[macro_export]
macro_rules! point {
    // Empty pattern - default point at origin
    () => {
        Point::new(0.0, 0.0, 0.0)
    };
    
    // Basic pattern with x, y, z coordinates
    ($x:expr, $y:expr, $z:expr) => {
        Point::new($x, $y, $z)
    };
    
    // Pattern with custom name
    ($x:expr, $y:expr, $z:expr, name: $name:expr) => {
        {
            let mut pt = Point::new($x, $y, $z);
            pt.data = Data::with_name($name);
            pt
        }
    };
}

/// Macro for creating a Vector with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use openmodel::geometry::Vector;
/// use openmodel::vector;
/// use openmodel::common::Data;
/// 
/// // Default vector (zero vector)
/// let v1 = vector![];
/// assert_eq!(v1.x, 0.0);
/// assert_eq!(v1.y, 0.0);
/// assert_eq!(v1.z, 0.0);
/// 
/// // Vector with components
/// let v2 = vector![1.0, 2.0, 3.0];
/// assert_eq!(v2.x, 1.0);
/// assert_eq!(v2.y, 2.0);
/// assert_eq!(v2.z, 3.0);
/// 
/// // Predefined axes
/// let x_axis = vector![x_axis];
/// assert_eq!(x_axis.x, 1.0);
/// assert_eq!(x_axis.y, 0.0);
/// assert_eq!(x_axis.z, 0.0);
/// 
/// let y_axis = vector![y_axis];
/// assert_eq!(y_axis.x, 0.0);
/// assert_eq!(y_axis.y, 1.0);
/// assert_eq!(y_axis.z, 0.0);
/// 
/// let z_axis = vector![z_axis];
/// assert_eq!(z_axis.x, 0.0);
/// assert_eq!(z_axis.y, 0.0);
/// assert_eq!(z_axis.z, 1.0);
/// 
/// // Vector with custom name
/// let v3 = vector![4.0, 5.0, 6.0, name: "CustomVector"];
/// assert_eq!(v3.x, 4.0);
/// assert_eq!(v3.y, 5.0);
/// assert_eq!(v3.z, 6.0);
/// assert_eq!(v3.data.name(), "CustomVector");
/// ```
#[macro_export]
macro_rules! vector {
    // Empty pattern - default zero vector
    () => {
        Vector::new(0.0, 0.0, 0.0)
    };
    
    // Basic pattern with x, y, z components
    ($x:expr, $y:expr, $z:expr) => {
        Vector::new($x, $y, $z)
    };
    
    // Predefined axes
    (x_axis) => {
        Vector::x_axis()
    };
    
    (y_axis) => {
        Vector::y_axis()
    };
    
    (z_axis) => {
        Vector::z_axis()
    };
    
    // Pattern with custom name
    ($x:expr, $y:expr, $z:expr, name: $name:expr) => {
        {
            let mut vec = Vector::new($x, $y, $z);
            vec.data = Data::with_name($name);
            vec
        }
    };
}

/// Macro for creating a Line with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use openmodel::geometry::Line;
/// use openmodel::line;
/// use openmodel::common::Data;
/// 
/// // Default line
/// let l1 = line![];
/// assert_eq!(l1.x0, 0.0);
/// assert_eq!(l1.y0, 0.0);
/// assert_eq!(l1.z0, 0.0);
/// assert_eq!(l1.x1, 1.0);
/// assert_eq!(l1.y1, 0.0);
/// assert_eq!(l1.z1, 0.0);
/// 
/// // Line with coordinates
/// let l2 = line![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
/// assert_eq!(l2.x0, 0.0);
/// assert_eq!(l2.y0, 0.0);
/// assert_eq!(l2.z0, 0.0);
/// assert_eq!(l2.x1, 1.0);
/// assert_eq!(l2.y1, 1.0);
/// assert_eq!(l2.z1, 1.0);
/// 
/// // Line with custom name
/// let l3 = line![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, name: "CustomLine"];
/// assert_eq!(l3.x0, 1.0);
/// assert_eq!(l3.y0, 2.0);
/// assert_eq!(l3.z0, 3.0);
/// assert_eq!(l3.x1, 4.0);
/// assert_eq!(l3.y1, 5.0);
/// assert_eq!(l3.z1, 6.0);
/// assert_eq!(l3.data.name(), "CustomLine");
/// ```
#[macro_export]
macro_rules! line {
    // Empty pattern - default line along X axis
    () => {
        Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    };
    
    // Line from coordinates
    ($x0:expr, $y0:expr, $z0:expr, $x1:expr, $y1:expr, $z1:expr) => {
        Line::new($x0, $y0, $z0, $x1, $y1, $z1)
    };
    
    // Line with custom name
    ($x0:expr, $y0:expr, $z0:expr, $x1:expr, $y1:expr, $z1:expr, name: $name:expr) => {
        {
            let mut line = Line::new($x0, $y0, $z0, $x1, $y1, $z1);
            line.data = Data::with_name($name);
            line
        }
    };
}

/// Macro for creating a Plane with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use openmodel::geometry::{Plane, Point, Vector};
/// use openmodel::plane;
/// use openmodel::common::Data;
/// 
/// // Default plane (XY plane at origin)
/// let p1 = plane![];
/// assert_eq!(p1.origin.x, 0.0);
/// assert_eq!(p1.origin.y, 0.0);
/// assert_eq!(p1.origin.z, 0.0);
/// assert_eq!(p1.xaxis.x, 1.0);
/// assert_eq!(p1.xaxis.y, 0.0);
/// assert_eq!(p1.xaxis.z, 0.0);
/// assert_eq!(p1.yaxis.x, 0.0);
/// assert_eq!(p1.yaxis.y, 1.0);
/// assert_eq!(p1.yaxis.z, 0.0);
/// assert_eq!(p1.zaxis.x, 0.0);
/// assert_eq!(p1.zaxis.y, 0.0);
/// assert_eq!(p1.zaxis.z, 1.0);
/// 
/// // Plane with specified origin, x-axis and y-axis
/// let p2 = plane![origin: (1.0, 2.0, 3.0), xaxis: (1.0, 0.0, 0.0), yaxis: (0.0, 1.0, 0.0)];
/// assert_eq!(p2.origin.x, 1.0);
/// assert_eq!(p2.origin.y, 2.0);
/// assert_eq!(p2.origin.z, 3.0);
/// assert_eq!(p2.xaxis.x, 1.0);
/// assert_eq!(p2.yaxis.y, 1.0);
/// assert_eq!(p2.zaxis.z, 1.0);
/// 
/// // Plane from existing point and vectors
/// let point = Point::new(4.0, 5.0, 6.0);
/// let x_vec = Vector::new(1.0, 0.0, 0.0);
/// let y_vec = Vector::new(0.0, 1.0, 0.0);
/// let p3 = plane![origin: point, xaxis: x_vec, yaxis: y_vec];
/// assert_eq!(p3.origin.x, 4.0);
/// assert_eq!(p3.origin.y, 5.0);
/// assert_eq!(p3.origin.z, 6.0);
/// assert_eq!(p3.xaxis.x, 1.0);
/// assert_eq!(p3.yaxis.y, 1.0);
/// assert_eq!(p3.zaxis.z, 1.0);
/// 
/// // Plane with custom name
/// let p4 = plane![origin: (1.0, 1.0, 1.0), xaxis: (1.0, 0.0, 0.0), yaxis: (0.0, 1.0, 0.0), name: "CustomPlane"];
/// assert_eq!(p4.origin.x, 1.0);
/// assert_eq!(p4.xaxis.x, 1.0);
/// assert_eq!(p4.data.name(), "CustomPlane");
/// ```
#[macro_export]
macro_rules! plane {
    // Empty pattern - default XY plane at origin
    () => {
        Plane::default()
    };
    
    // Basic pattern with origin point, x-axis and y-axis
    (origin: ($x:expr, $y:expr, $z:expr), xaxis: ($xx:expr, $xy:expr, $xz:expr), yaxis: ($yx:expr, $yy:expr, $yz:expr)) => {
        Plane::new(
            Point::new($x, $y, $z), 
            Vector::new($xx, $xy, $xz), 
            Vector::new($yx, $yy, $yz)
        )
    };
    
    // From existing point and vectors
    (origin: $origin:expr, xaxis: $xaxis:expr, yaxis: $yaxis:expr) => {
        Plane::new($origin, $xaxis, $yaxis)
    };
    
    // Pattern with custom name
    (origin: ($x:expr, $y:expr, $z:expr), xaxis: ($xx:expr, $xy:expr, $xz:expr), yaxis: ($yx:expr, $yy:expr, $yz:expr), name: $name:expr) => {
        {
            let mut pln = Plane::new(
                Point::new($x, $y, $z), 
                Vector::new($xx, $xy, $xz), 
                Vector::new($yx, $yy, $yz)
            );
            pln.data = Data::with_name($name);
            pln
        }
    };
}

/// Macro for creating a Color with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use openmodel::geometry::Color;
/// use openmodel::color;
/// use openmodel::common::Data;
/// 
/// // Default color (black)
/// let c1 = color![];
/// assert_eq!(c1.r, 0);
/// assert_eq!(c1.g, 0);
/// assert_eq!(c1.b, 0);
/// assert_eq!(c1.a, 0);
/// 
/// // RGB color (with default alpha = 0)
/// let c2 = color![100, 150, 200];
/// assert_eq!(c2.r, 100);
/// assert_eq!(c2.g, 150);
/// assert_eq!(c2.b, 200);
/// assert_eq!(c2.a, 0); 
/// 
/// // RGBA color
/// let c3 = color![50, 100, 150, 255];
/// assert_eq!(c3.r, 50);
/// assert_eq!(c3.g, 100);
/// assert_eq!(c3.b, 150);
/// assert_eq!(c3.a, 255);
/// 
/// // Named colors
/// let red = color![red];
/// assert_eq!(red.r, 255);
/// assert_eq!(red.g, 0);
/// assert_eq!(red.b, 0);
/// assert_eq!(red.a, 255);
/// 
/// let green = color![green];
/// assert_eq!(green.r, 0);
/// assert_eq!(green.g, 255);
/// assert_eq!(green.b, 0);
/// assert_eq!(green.a, 255);
/// 
/// let blue = color![blue];
/// assert_eq!(blue.r, 0);
/// assert_eq!(blue.g, 0);
/// assert_eq!(blue.b, 255);
/// assert_eq!(blue.a, 255);
/// 
/// // With custom name
/// let custom = color![255, 128, 64, 192, name: "CustomColor"];
/// assert_eq!(custom.r, 255);
/// assert_eq!(custom.g, 128);
/// assert_eq!(custom.b, 64);
/// assert_eq!(custom.a, 192);
/// assert_eq!(custom.data.name(), "CustomColor");
/// ```
#[macro_export]
macro_rules! color {
    // Empty pattern - default black color
    () => {
        Color::new(0, 0, 0, 0)
    };
    
    // RGB color with default alpha
    ($r:expr, $g:expr, $b:expr) => {
        Color::new($r, $g, $b, 0)
    };
    
    // RGBA color
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::new($r, $g, $b, $a)
    };
    
    // Named colors
    (red) => {
        Color::new(255, 0, 0, 255)
    };
    
    (green) => {
        Color::new(0, 255, 0, 255)
    };
    
    (blue) => {
        Color::new(0, 0, 255, 255)
    };
    
    (black) => {
        Color::new(0, 0, 0, 255)
    };
    
    (white) => {
        Color::new(255, 255, 255, 255)
    };
    
    (yellow) => {
        Color::new(255, 255, 0, 255)
    };
    
    (cyan) => {
        Color::new(0, 255, 255, 255)
    };
    
    (magenta) => {
        Color::new(255, 0, 255, 255)
    };
    
    // With custom name
    ($r:expr, $g:expr, $b:expr, $a:expr, name: $name:expr) => {
        {
            let mut clr = Color::new($r, $g, $b, $a);
            clr.data = Data::with_name($name);
            clr
        }
    };
}

/// Macro for creating an Xform with a more concise syntax
/// 
/// # Examples
/// 
/// ```
/// use openmodel::geometry::Xform;
/// use openmodel::xform;
/// use openmodel::common::Data;
/// 
/// // Identity matrix
/// let x1 = xform![];
/// assert_eq!(x1.m00, 1.0);
/// assert_eq!(x1.m11, 1.0);
/// assert_eq!(x1.m22, 1.0);
/// assert_eq!(x1.m33, 1.0);
/// assert_eq!(x1.m01, 0.0);
/// 
/// // Translation
/// let x2 = xform![translation: 1.0, 2.0, 3.0];
/// assert_eq!(x2.m03, 1.0); // tx
/// assert_eq!(x2.m13, 2.0); // ty
/// assert_eq!(x2.m23, 3.0); // tz
/// assert_eq!(x2.m00, 1.0); // Identity part
/// 
/// // Scaling
/// let x3 = xform![scaling: 2.0, 3.0, 4.0];
/// assert_eq!(x3.m00, 2.0); // sx
/// assert_eq!(x3.m11, 3.0); // sy
/// assert_eq!(x3.m22, 4.0); // sz
/// 
/// // Rotation around X (testing approximate values due to floating point)
/// let x4 = xform![rotation_x: 90.0]; // 90 degrees
/// assert!((x4.m11 - 0.0).abs() < 1e-10); // cos(90°)
/// assert!((x4.m12 - (-1.0)).abs() < 1e-10); // -sin(90°)
/// assert!((x4.m21 - 1.0).abs() < 1e-10); // sin(90°)
/// assert!((x4.m22 - 0.0).abs() < 1e-10); // cos(90°)
/// 
/// // Full matrix specification
/// let x5 = xform![
///     1.0, 2.0, 3.0, 4.0,
///     5.0, 6.0, 7.0, 8.0,
///     9.0, 10.0, 11.0, 12.0,
///     13.0, 14.0, 15.0, 16.0
/// ];
/// assert_eq!(x5.m00, 1.0);
/// assert_eq!(x5.m01, 2.0);
/// assert_eq!(x5.m13, 8.0);
/// assert_eq!(x5.m33, 16.0);
/// 
/// // With custom name
/// let x6 = xform![name: "MyTransform"];
/// assert_eq!(x6.data.name(), "MyTransform");
/// ```
#[macro_export]
macro_rules! xform {
    // Pattern for identity matrix
    () => {
        Xform::new()
    };
    
    // Pattern with custom name
    (name: $name:expr) => {
        {
            let mut xf = Xform::new();
            xf.data = Data::with_name($name);
            xf
        }
    };
    
    // Pattern for translation
    (translation: $tx:expr, $ty:expr, $tz:expr) => {
        Xform::translation($tx, $ty, $tz)
    };
    
    // Pattern for scaling
    (scaling: $sx:expr, $sy:expr, $sz:expr) => {
        Xform::scaling($sx, $sy, $sz)
    };
    
    // Pattern for rotation around X axis
    (rotation_x: $angle:expr) => {
        {
            let angle_rad = $angle * std::f64::consts::PI / 180.0;
            let s = angle_rad.sin();
            let c = angle_rad.cos();
            
            Xform {
                m00: 1.0, m01: 0.0, m02: 0.0, m03: 0.0,
                m10: 0.0, m11: c,   m12: -s,  m13: 0.0,
                m20: 0.0, m21: s,   m22: c,   m23: 0.0,
                m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
                data: Data::with_name("XRotation")
            }
        }
    };
    
    // Pattern for rotation around Y axis
    (rotation_y: $angle:expr) => {
        {
            let angle_rad = $angle * std::f64::consts::PI / 180.0;
            let s = angle_rad.sin();
            let c = angle_rad.cos();
            
            Xform {
                m00: c,   m01: 0.0, m02: s,   m03: 0.0,
                m10: 0.0, m11: 1.0, m12: 0.0, m13: 0.0,
                m20: -s,  m21: 0.0, m22: c,   m23: 0.0,
                m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
                data: Data::with_name("YRotation")
            }
        }
    };
    
    // Pattern for rotation around Z axis
    (rotation_z: $angle:expr) => {
        {
            let angle_rad = $angle * std::f64::consts::PI / 180.0;
            let s = angle_rad.sin();
            let c = angle_rad.cos();
            
            Xform {
                m00: c,   m01: -s,  m02: 0.0, m03: 0.0,
                m10: s,   m11: c,   m12: 0.0, m13: 0.0,
                m20: 0.0, m21: 0.0, m22: 1.0, m23: 0.0,
                m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
                data: Data::with_name("ZRotation")
            }
        }
    };
    
    // Pattern for full matrix specification
    ($m00:expr, $m01:expr, $m02:expr, $m03:expr,
     $m10:expr, $m11:expr, $m12:expr, $m13:expr,
     $m20:expr, $m21:expr, $m22:expr, $m23:expr,
     $m30:expr, $m31:expr, $m32:expr, $m33:expr) => {
        Xform {
            m00: $m00, m01: $m01, m02: $m02, m03: $m03,
            m10: $m10, m11: $m11, m12: $m12, m13: $m13,
            m20: $m20, m21: $m21, m22: $m22, m23: $m23,
            m30: $m30, m31: $m31, m32: $m32, m33: $m33,
            data: Data::with_name("CustomMatrix")
        }
    };
}
