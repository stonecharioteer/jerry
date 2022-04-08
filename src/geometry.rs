//! module to hold all the geometrical functions and definitions
use std::{f32::consts::PI, str::FromStr};
/// A geometrical point
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn generate_spiral(&self, arc: Option<u8>, separation: Option<u8>) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        let arc = match arc {
            Some(v) => v,
            None => 2,
        };
        let separation = match separation {
            Some(v) => v,
            None => 2,
        };

        let mut r = arc as f32;
        let b: f32 = separation as f32 / (2.0 * PI);
        let mut phi = r as f32 / b;
        for _ in 1..100 {
            points.push(Point {
                x: r as i32,
                y: phi as i32,
            });
            phi = phi + (arc as f32 / r as f32);
            r = b * phi;
        }
        points
    }

    fn polar_to_cartesian(radius: u16, phi: f32) -> (f32, f32) {
        (radius as f32 * PI.cos(), radius as f32 * PI.sin())
    }
}
/// This is an enum to just hold the valid direction values
#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Implement the `from_str` function for the Direction enum so that the CLI can read the
// input and build a Direction value from it.
impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_l: &str = &s.to_lowercase()[..];
        match s_l {
            "left" => Ok(Self::Left),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "right" => Ok(Self::Right),
            e => Err(format!("`{e}` is not an accepted value.")),
        }
    }
}

/// This struct provides the bounding box
/// for a given monitor
pub struct BoundingBox {
    top_left: Point,
    top_right: Point,
    bottom_left: Point,
    bottom_right: Point,
}

impl BoundingBox {
    /// construct a bounding box for a given monitor
    pub fn new(m: &impl Rectangle) -> Self {
        Self {
            top_left: m.top_left(),
            top_right: m.top_right(),
            bottom_left: m.bottom_left(),
            bottom_right: m.bottom_right(),
        }
    }
}

pub trait Rectangle {
    fn contains_point(&self, x: i32, y: i32) -> bool;
    fn top_left(&self) -> Point;
    fn top_right(&self) -> Point;
    fn bottom_left(&self) -> Point;
    fn bottom_right(&self) -> Point;
    fn bounding_box(&self) -> BoundingBox;
}
