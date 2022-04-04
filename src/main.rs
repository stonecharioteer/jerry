//! This is a CLI that allows me to move my mouse to a particular monitor in a
//! multi-monitor setup.

use jerry::cli::cli;

fn main() {
    env_logger::init();
    cli();
}
