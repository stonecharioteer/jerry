use crate::geometry::{Direction, Rectangle};
use crate::monitor::structs::CustomMonitor;
use log::{debug, info};
use mouse_rs::Mouse;
use xrandr::{Monitor, XHandle, XrandrError};

/// this function lists monitors
pub fn list_monitors() -> Result<Vec<Monitor>, XrandrError> {
    let monitors = XHandle::open()?.monitors()?;
    return Ok(monitors);
}

/// This function moves the mouse to the monitor
pub fn move_to_monitor(required_monitor: String) -> Result<(), clap::Error> {
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

/// This function figures out which monitor the mouse is in.
pub fn which_monitor_is_mouse_in() -> Result<Monitor, clap::Error> {
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

/// This function moves the focus in the specified direction.
pub fn move_in_direction(
    direction: &Direction,
    wrap_around: Option<bool>,
) -> Result<(), clap::Error> {
    let current_monitor = which_monitor_is_mouse_in().unwrap();
    let wrap_around = match wrap_around {
        Some(v) => v,
        None => false,
    };
    let next_monitor = current_monitor.next(&direction, wrap_around);
    match next_monitor {
        Ok(v) => move_to_monitor(v.name),
        Err(_) => match wrap_around {
            true => Err(clap::Error::raw(clap::ErrorKind::InvalidValue, "There is no next monitor.")),
            false => Err(clap::Error::raw(clap::ErrorKind::DisplayHelp, "There is no next monitor in this direction. Perhaps you'd want to use the --wrap-around argument?")),
        }
    }
}
