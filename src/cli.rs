use crate::geometry::Direction;
use crate::monitor::utils;
use log::{debug, info};
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

    #[structopt(short, long)]
    wrap_around: bool,
}

pub fn cli() {
    let opt = Opt::from_args();
    let monitor = utils::which_monitor_is_mouse_in().unwrap();
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
        // TODO : move all the clap::Error calls here instead of within the functions
        (Some(monitor), None) => {
            info!("Attempting to move to monitor: {monitor}");
            let res = utils::move_to_monitor(monitor.to_owned());
            match res {
                Ok(_) => return,
                Err(e) => e.exit(),
            }
        }
        (None, Some(direction)) => {
            info!("Attempting to move in direction: {direction:?}");
            let res = utils::move_in_direction(&direction, Some(opt.wrap_around));
            match res {
                Ok(_) => return,
                Err(e) => e.exit(),
            }
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
