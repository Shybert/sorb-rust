fn main() {
    println!("Hello, world!");
}

fn approx_equals(a: f64, b: f64) -> bool {
    return (a - b).abs() < 0.00001;
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        return approx_equals(self.x, other.x)
            && approx_equals(self.y, other.y)
            && approx_equals(self.z, other.z);
    }
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl PartialEq<Vector> for Vector {
    fn eq(&self, other: &Vector) -> bool {
        return approx_equals(self.x, other.x)
            && approx_equals(self.y, other.y)
            && approx_equals(self.z, other.z)
            && approx_equals(self.w, other.w);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_equality() {
        assert_eq!(
            Point {
                x: 4.0,
                y: -4.0,
                z: 3.0
            },
            Point {
                x: 4.0,
                y: -4.0,
                z: 3.0
            }
        );

        assert_eq!(
            Point {
                x: 1.000000001,
                y: 0.0,
                z: 0.0
            },
            Point {
                x: 1.0,
                y: 0.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn vector_equality() {
        assert_eq!(
            Vector {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: -42.0
            },
            Vector {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: -42.0
            }
        );

        assert_eq!(
            Vector {
                x: 1.000000001,
                y: 0.0,
                z: 0.0,
                w: 0.0
            },
            Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0
            }
        );
    }
}
