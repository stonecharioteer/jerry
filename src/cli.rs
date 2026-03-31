use crate::geometry::Direction;
use crate::monitor::utils;
use log::info;
use structopt::StructOpt;

/// Jerry moves your mouse pointer to a specific monitor or in a direction across monitors.
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

    /// Jiggle the pointer around the target point before final placement.
    #[structopt(short, long)]
    animate_mouse: bool,

    /// Scroll down and up once after moving the pointer.
    #[structopt(short = "s", long)]
    scroll_wheel: bool,

    #[structopt(short, long)]
    wrap_around: bool,
}

pub fn cli() {
    let opt = Opt::from_args();
    let effects = utils::PointerEffects {
        jiggle: opt.animate_mouse,
        scroll: opt.scroll_wheel,
    };
    let monitor_name = match utils::current_monitor_name() {
        Ok(name) => name,
        Err(e) => e.exit(),
    };
    info!("Mouse is currently in {monitor_name}");
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
            let res = utils::move_to_monitor(monitor.to_owned(), effects);
            match res {
                Ok(_) => return,
                Err(e) => e.exit(),
            }
        }
        (None, Some(direction)) => {
            info!("Attempting to move in direction: {direction:?}");
            let res = utils::move_in_direction(&direction, Some(opt.wrap_around), effects);
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
