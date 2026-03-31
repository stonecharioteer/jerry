use crate::geometry::{Direction, Rectangle};
use crate::monitor::backend;
use crate::monitor::structs::{build_adjacency_graph, MonitorInfo};
use log::{debug, info};
use mouse_rs::Mouse;

#[derive(Debug, Clone, Copy, Default)]
pub struct PointerEffects {
    pub jiggle: bool,
    pub scroll: bool,
}

fn backend_error_to_clap(err: backend::BackendError) -> clap::Error {
    clap::Error::raw(clap::ErrorKind::InvalidValue, err.to_string())
}

fn mouse_error_to_clap(context: &str, err: impl std::fmt::Display) -> clap::Error {
    clap::Error::raw(clap::ErrorKind::InvalidValue, format!("{context}: {err}"))
}

pub fn list_monitors() -> Result<Vec<MonitorInfo>, clap::Error> {
    backend::list_monitors().map_err(backend_error_to_clap)
}

pub fn current_monitor_name() -> Result<String, clap::Error> {
    let monitors = list_monitors()?;
    let monitor_index = which_monitor_is_mouse_in(&monitors)?;
    Ok(monitors[monitor_index].name.clone())
}

pub fn move_to_monitor(
    required_monitor: String,
    effects: PointerEffects,
) -> Result<(), clap::Error> {
    let monitors = list_monitors()?;
    let monitor_names: Vec<String> = monitors
        .iter()
        .map(|monitor| monitor.name.clone())
        .collect();
    let Some(matching_monitor) = monitors
        .iter()
        .find(|monitor| monitor.name == required_monitor)
    else {
        return Err(clap::Error::raw(
            clap::ErrorKind::InvalidValue,
            format!(
                "Unable to find the monitor: `{required_monitor}`. \
                    Available monitors are: {monitor_names:?}"
            ),
        ));
    };

    debug!("Matching monitor found: {matching_monitor:?}");
    move_to_monitor_info(matching_monitor, effects)
}

pub fn move_in_direction(
    direction: &Direction,
    wrap_around: Option<bool>,
    effects: PointerEffects,
) -> Result<(), clap::Error> {
    let monitors = list_monitors()?;
    let current_index = which_monitor_is_mouse_in(&monitors)?;
    let wrap_around = wrap_around.unwrap_or(false);

    let graph = build_adjacency_graph(&monitors, wrap_around);
    let Some(next_index) = graph.get(&(current_index, *direction)).copied() else {
        return match wrap_around {
            true => Err(clap::Error::raw(
                clap::ErrorKind::InvalidValue,
                "There is no next monitor.",
            )),
            false => Err(clap::Error::raw(
                clap::ErrorKind::DisplayHelp,
                "There is no next monitor in this direction. Perhaps you'd want to use the --wrap-around argument?",
            )),
        };
    };

    let next_monitor = &monitors[next_index];
    info!(
        "Moving to monitor {} in direction {direction:?}",
        next_monitor.name
    );
    move_to_monitor_info(next_monitor, effects)
}

fn which_monitor_is_mouse_in(monitors: &[MonitorInfo]) -> Result<usize, clap::Error> {
    let mouse = Mouse::new();
    let position = mouse
        .get_position()
        .map_err(|err| mouse_error_to_clap("Unable to read the mouse pointer position", err))?;

    for (index, monitor) in monitors.iter().enumerate() {
        if monitor.contains_point(position.x, position.y) {
            return Ok(index);
        }
    }

    Err(clap::Error::raw(
        clap::ErrorKind::UnknownArgument,
        "Unable to find which monitor contains the mouse. This should not occur!",
    ))
}

fn move_to_monitor_info(monitor: &MonitorInfo, effects: PointerEffects) -> Result<(), clap::Error> {
    let mouse = Mouse::new();
    let x0 = monitor.left();
    let y0 = monitor.top();
    let x1 = monitor.right();
    let y1 = monitor.bottom();
    let (x, y) = ((x1 + x0) / 2, (y1 + y0) / 2);

    info!("Moving mouse to {},{} on monitor `{}`", x, y, monitor.name);

    if effects.jiggle {
        mouse
            .move_to(x + 10, y + 10)
            .map_err(|err| mouse_error_to_clap("Unable to move pointer", err))?;
        mouse
            .move_to(x - 10, y - 10)
            .map_err(|err| mouse_error_to_clap("Unable to move pointer", err))?;
    }

    mouse
        .move_to(x, y)
        .map_err(|err| mouse_error_to_clap("Unable to move pointer", err))?;

    if effects.scroll {
        mouse
            .wheel(1)
            .map_err(|err| mouse_error_to_clap("Unable to scroll pointer wheel", err))?;
        mouse
            .wheel(-1)
            .map_err(|err| mouse_error_to_clap("Unable to scroll pointer wheel", err))?;
    }

    Ok(())
}
