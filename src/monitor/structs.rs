//! This module implements the various structs and traits for the xrandr monitor class
use xrandr::Monitor;

use crate::geometry::{BoundingBox, Direction, Point, Rectangle};
use log::{debug, info};
/// This is a trait that I'm implementing atop of the default xrandr monitor struct
/// so I can use additional methods that are very particular to what I'm trying to do.
pub trait CustomMonitor {
    fn next(&self, direction: &Direction, wrap_around: bool) -> Result<Monitor, String>;
}

impl Rectangle for Monitor {
    /// returns true if the current monitor contains a point
    fn contains_point(&self, x: i32, y: i32) -> bool {
        let x0 = self.x;
        let y0 = self.y;
        let x1 = self.x + self.width_px;
        let y1 = self.y + self.height_px;
        (x >= x0) && (x <= x1) && (y >= y0) && (y <= y1)
    }

    /// get the top left corner
    fn top_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    /// get the top right corner
    fn top_right(&self) -> Point {
        Point {
            x: self.x + self.width_px,
            y: self.y,
        }
    }

    /// get the bottom left corner
    fn bottom_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + self.height_px,
        }
    }

    /// get the bottom right corner
    fn bottom_right(&self) -> Point {
        Point {
            x: self.x + self.width_px,
            y: self.y + self.height_px,
        }
    }

    /// get the bounding box for the monitor
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self)
    }
}

impl CustomMonitor for Monitor {
    /// Returns the next monitor in a direction
    fn next(&self, direction: &Direction, wrap_around: bool) -> Result<Monitor, String> {
        match direction {
            Direction::Up => {
                debug!("Moving up");
                // figure out the positions of each monitor.
                // Given the monitor `current_monitor`,
                // figure out where to move next.
                let next_monitor = self.next(&direction, wrap_around);
                todo!("Need to implement this");
            }

            Direction::Down => {
                debug!("Moving down");
                todo!("Need to implement this");
            }
            Direction::Left => {
                debug!("Moving left");
                todo!("Need to implement this");
            }
            Direction::Right => {
                debug!("Moving right");
                todo!("Need to implement this");
            }
        }
    }
}
