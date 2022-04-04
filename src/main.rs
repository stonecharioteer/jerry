// This is a CLI that allows me to move my mouse to a particular monitor in a
// multi-monitor setup.

use std::str::FromStr;

use clap::{self, Error, ErrorKind};
use log::debug;
use structopt::StructOpt;
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
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
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

fn move_to_monitor(monitor: String) {

}

fn move_in_direction(direction: &Direction) {

}

fn main() {
    let opt = Opt::from_args();

    match (&opt.monitor, &opt.direction) {
        (None, None) => {
            Error::raw(
                ErrorKind::TooFewValues,
                "You need to specify either the direction \
                or the monitor into which you'd want to move.",
            )
            .exit();
        }
        (Some(monitor), None) => {
            debug!("Attempting to move to monitor: {monitor}");
            move_to_monitor(monitor.to_owned());
        }
        (None, Some(direction)) => {
            debug!("Attempting to move in direction: {direction:?}");
            move_in_direction(&direction);
        }
        (Some(_), Some(_)) => {
            Error::raw(
                ErrorKind::TooManyValues,
                "You can only specify *one* of the fields, not both.",
            )
            .exit();
        }
    }
}
