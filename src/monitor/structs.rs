//! This module implements the various structs and traits for the xrandr monitor class
use xrandr::Monitor;

use crate::geometry::{BoundingBox, Direction, Point, Rectangle};
use log::{debug, info};

use super::utils::list_monitors;

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

/// This is a trait that I'm implementing atop of the default xrandr monitor struct
/// so I can use additional methods that are very particular to what I'm trying to do.
pub trait CustomMonitor {
    fn next(&self, direction: &Direction, wrap_around: bool) -> Result<Monitor, String>;
}

impl CustomMonitor for Monitor {
    /// Returns the next monitor in a direction
    fn next(&self, direction: &Direction, wrap_around: bool) -> Result<Monitor, String> {
        let monitors: Vec<Monitor> = list_monitors()
            .unwrap()
            .into_iter()
            .filter(|m| m.name != self.name)
            .collect();
        match direction {
            Direction::Up => {
                debug!("Moving up");
                // calculate the distance to the bottom of each monitor to the *top* of the
                // current monitor
                for monitor in monitors {
                    let y_difference = &monitor.y + &monitor.height_px - self.y;
                    let name = &monitor.name;
                    debug!("Distance from top of current to bottom of {name}: {y_difference}");
                    if y_difference == 0 {
                        return Ok(monitor);
                    }
                }
                if wrap_around {
                    unimplemented!()
                } else {
                    return Err("There is no monitor above this one.".to_owned());
                }
            }
            Direction::Down => {
                debug!("Moving down");
                // calculate the distance to the bottom of each monitor to the *top* of the
                // current monitor
                for monitor in monitors {
                    let y_difference = &monitor.y - (self.y + self.height_px);
                    let name = &monitor.name;
                    debug!("Distance from bottom of current to top of {name}: {y_difference}");
                    if y_difference == 0 {
                        return Ok(monitor);
                    }
                }
                if wrap_around {
                    unimplemented!()
                } else {
                    return Err("There is no monitor below this one.".to_owned());
                }
            }
            Direction::Left => {
                debug!("Moving left");
                // calculate the distance to the right of each monitor to the *left* of the
                // current monitor
                for monitor in monitors {
                    let x_difference = &monitor.x + monitor.width_px - self.x;
                    let name = &monitor.name;
                    debug!("Distance from left side of current to right of {name}: {x_difference}");
                    if x_difference == 0 {
                        return Ok(monitor);
                    }
                }
                if wrap_around {
                    unimplemented!()
                } else {
                    return Err("There is no monitor on the left of this one.".to_owned());
                }
            }
            Direction::Right => {
                debug!("Moving right");
                // calculate the distance to the left of each monitor to the *right* of the
                // current monitor
                for monitor in monitors {
                    let x_difference = &monitor.x - (self.x + self.width_px);
                    let name = &monitor.name;
                    debug!("Distance from right side of current to left of {name}: {x_difference}");
                    if x_difference == 0 {
                        return Ok(monitor);
                    }
                }
                if wrap_around {
                    unimplemented!()
                } else {
                    return Err("There is no monitor on the left of this one.".to_owned());
                }
            }
        }
    }
}
