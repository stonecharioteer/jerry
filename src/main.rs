//! This is a CLI that allows me to move my mouse to a particular monitor in a
//! multi-monitor setup.

use clap;
use jerry::cli::Opt;
use jerry::monitor::utils;
use log::{debug, info};
use structopt::StructOpt;

fn main() {
    env_logger::init();
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
            utils::move_in_direction(&direction, Some(false));
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
