// This is a CLI that allows me to move my mouse to a particular monitor in a
// multi-monitor setup.

use std::str::FromStr;

use clap;
use log::{debug, info};
use mouse_rs::Mouse;
use structopt::StructOpt;
use xrandr::{Monitor, XHandle, XrandrError};
/// Jerry is a tool that I wrote to help me move my mouse to a specific monitor when I'm using a
/// tiling window manager on Linux. Qtile doesn't seem to move the mouse focus to a specific
/// monitor when moving focus to a new monitor, that makes dmenu stick to the original monitor,
/// which is rather annoying.
#[derive(Debug, StructOpt)]
#[allow(dead_code)]
struct Opt {
    /// Monitor name. Use a configuration file to map the monitors to
    /// the names.
    #[structopt(short, long)]
    monitor: Option<String>,

    /// Which direction you'd like to move your mouse to.
    #[structopt(short, long)]
    direction: Option<Direction>,

    #[structopt(short, long)]
    wrap_around: bool,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

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

/// this function lists monitors
fn list_monitors() -> Result<Vec<Monitor>, XrandrError> {
    let monitors = XHandle::open()?.monitors()?;
    return Ok(monitors);
}

/// This function moves the mouse to the monitor
fn move_to_monitor(required_monitor: String) -> Result<(), clap::Error> {
    let monitor_names: Vec<String> = list_monitors()
        .unwrap()
        .into_iter()
        .map(|monitor| monitor.name)
        .collect();
    if monitor_names.contains(&required_monitor) {
        let matching_monitors: Vec<Monitor> = list_monitors()
            .unwrap()
            .into_iter()
            .filter(|monitor| monitor.name == required_monitor)
            .collect();
        let matching_monitor = &matching_monitors[0];
        debug!("Matching Monitor found: {matching_monitor:?}");
        let x0 = matching_monitor.x;
        let y0 = matching_monitor.y;
        let x1 = x0 + matching_monitor.width_px;
        let y1 = y0 + matching_monitor.height_px;
        let mouse = Mouse::new();
        let (x, y) = ((x1 + x0) / 2, (y1 + y0) / 2);
        info!("Moving Mouse, {x}, {y}!");
        _ = mouse.move_to(x, y);
        Ok(())
    } else {
        Err(clap::Error::raw(
            clap::ErrorKind::InvalidValue,
            format!(
                "Unable to find the monitor: `{required_monitor}`. \
                    Available monitors are: {monitor_names:?}"
            ),
        ))
    }
}

/// This function moves the focus in the specified direction.
fn move_in_direction(direction: &Direction, wrap_around: Option<bool>) {
    let current_monitor = which_monitor_is_mouse_in().unwrap();
    match direction {
        Direction::Up => {
            debug!("Moving up");
            // figure out the positions of each monitor.
            // Given the monitor `current_monitor`,
            // figure out where to move next.
        }
        Direction::Down => {
            debug!("Moving down");
        }
        Direction::Left => {
            debug!("Moving left");
        }
        Direction::Right => {
            debug!("Moving right");
        }
    }
}

struct Point {
    pub x: i32,
    pub y: i32,
}

trait CustomMonitor {
    fn contains_point(&self, x: i32, y: i32) -> bool;
    fn top_left(&self) -> Point;
    fn top_right(&self) -> Point;
    fn bottom_left(&self) -> Point;
    fn bottom_right(&self) -> Point;
    fn bounding_box(&self) -> BoundingBox;
}

struct BoundingBox {
    top_left: Point,
    top_right: Point,
    bottom_left: Point,
    bottom_right: Point,
}

impl BoundingBox {
    fn new(m: &Monitor) -> Self {
        Self {
            top_left: m.top_left(),
            top_right: m.top_right(),
            bottom_left: m.bottom_left(),
            bottom_right: m.bottom_right(),
        }
    }
}

impl CustomMonitor for Monitor {
    /// returns true if the current monitor contains a point
    fn contains_point(&self, x: i32, y: i32) -> bool {
        let x0 = self.x;
        let y0 = self.y;
        let x1 = self.x + self.width_px;
        let y1 = self.y + self.height_px;
        (x >= x0) && (x <= x1) && (y >= y0) && (y <= y1)
    }

    fn top_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
    fn top_right(&self) -> Point {
        Point {
            x: self.x + self.width_px,
            y: self.y,
        }
    }
    fn bottom_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + self.height_px,
        }
    }
    fn bottom_right(&self) -> Point {
        Point {
            x: self.x + self.width_px,
            y: self.y + self.height_px,
        }
    }
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(&self)
    }
}

/// This function figures out which monitor the mouse is in.
fn which_monitor_is_mouse_in() -> Result<Monitor, clap::Error> {
    let mouse = Mouse::new();
    let position = mouse.get_position().unwrap();
    for monitor in list_monitors().unwrap() {
        if monitor.contains_point(position.x, position.y) {
            return Ok(monitor);
        }
    }
    Err(clap::Error::raw(
        clap::ErrorKind::UnknownArgument,
        "Unable to find which monitor contains the mouse. \
        This should not occur!",
    ))
}

fn main() {
    env_logger::init();
    let opt = Opt::from_args();
    let monitor = which_monitor_is_mouse_in().unwrap();
    let monitor_name = &monitor.name;
    info!("Mouse is currently in {monitor_name}");
    debug!("current monitor = {monitor:?}");
    match (&opt.monitor, &opt.direction) {
        (None, None) => {
            clap::Error::raw(
                clap::ErrorKind::TooFewValues,
                "You need to specify either the direction \
                or the monitor into which you'd want to move.",
            )
            .exit();
        }
        (Some(monitor), None) => {
            info!("Attempting to move to monitor: {monitor}");
            let res = move_to_monitor(monitor.to_owned());
            match res {
                Ok(_) => return,
                Err(e) => e.exit(),
            }
        }
        (None, Some(direction)) => {
            info!("Attempting to move in direction: {direction:?}");
            move_in_direction(&direction, Some(false));
        }
        (Some(_), Some(_)) => {
            clap::Error::raw(
                clap::ErrorKind::TooManyValues,
                "You can only specify *one* of the fields, not both.",
            )
            .exit();
        }
    }
}
