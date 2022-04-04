use crate::geometry::Direction;
use structopt::StructOpt;

/// Jerry is a tool that I wrote to help me move my mouse to a specific monitor when I'm using a
/// tiling window manager on Linux. Qtile doesn't seem to move the mouse focus to a specific
/// monitor when moving focus to a new monitor, that makes dmenu stick to the original monitor,
/// which is rather annoying.
#[derive(Debug, StructOpt)]
#[allow(dead_code)]
pub struct Opt {
    /// Monitor name. Use a configuration file to map the monitors to
    /// the names.
    #[structopt(short, long)]
    pub monitor: Option<String>,

    /// Which direction you'd like to move your mouse to.
    #[structopt(short, long)]
    pub direction: Option<Direction>,

    #[structopt(short, long)]
    pub wrap_around: bool,
}
