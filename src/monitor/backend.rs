use std::fmt;

use super::structs::MonitorInfo;

#[derive(Debug, Clone)]
pub struct BackendError {
    message: String,
}

impl BackendError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BackendError {}

pub trait MonitorBackend {
    fn list_monitors() -> Result<Vec<MonitorInfo>, BackendError>;
}

#[cfg(target_os = "linux")]
mod platform {
    use super::{BackendError, MonitorBackend, MonitorInfo};
    use xrandr::XHandle;

    pub(super) struct PlatformBackend;

    impl MonitorBackend for PlatformBackend {
        fn list_monitors() -> Result<Vec<MonitorInfo>, BackendError> {
            let xh = XHandle::open()
                .map_err(|err| BackendError::new(format!("Unable to open X11 handle: {err}")))?;
            let monitors = xh.monitors().map_err(|err| {
                BackendError::new(format!("Unable to query monitors from xrandr: {err}"))
            })?;
            Ok(monitors
                .into_iter()
                .map(|monitor| {
                    MonitorInfo::new(
                        monitor.name,
                        monitor.x,
                        monitor.y,
                        monitor.width_px,
                        monitor.height_px,
                    )
                })
                .collect())
        }
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use super::{BackendError, MonitorBackend, MonitorInfo};
    use core_graphics::display::CGDisplay;

    pub(super) struct PlatformBackend;

    impl MonitorBackend for PlatformBackend {
        fn list_monitors() -> Result<Vec<MonitorInfo>, BackendError> {
            let displays = CGDisplay::active_displays().map_err(|err| {
                BackendError::new(format!("Unable to query active displays via Quartz: {err}"))
            })?;
            if displays.is_empty() {
                return Err(BackendError::new("No active displays found."));
            }

            Ok(displays
                .into_iter()
                .map(|display_id| {
                    let display = CGDisplay::new(display_id);
                    let bounds = display.bounds();
                    MonitorInfo::new(
                        format!("display-{display_id}"),
                        bounds.origin.x.round() as i32,
                        bounds.origin.y.round() as i32,
                        bounds.size.width.round() as i32,
                        bounds.size.height.round() as i32,
                    )
                })
                .collect())
        }
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use std::{mem, ptr};

    use super::{BackendError, MonitorBackend, MonitorInfo};
    use winapi::shared::minwindef::{BOOL, LPARAM, TRUE};
    use winapi::shared::windef::{HDC, HMONITOR, LPRECT};
    use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFOEXW};

    pub(super) struct PlatformBackend;

    unsafe extern "system" fn enum_monitor_proc(
        monitor: HMONITOR,
        _hdc: HDC,
        _clip_rect: LPRECT,
        lparam: LPARAM,
    ) -> BOOL {
        let monitors = &mut *(lparam as *mut Vec<MonitorInfo>);

        let mut info: MONITORINFOEXW = mem::zeroed();
        info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
        if GetMonitorInfoW(monitor, &mut info as *mut MONITORINFOEXW as *mut _) == 0 {
            return TRUE;
        }

        let name_len = info
            .szDevice
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(info.szDevice.len());
        let name = String::from_utf16_lossy(&info.szDevice[..name_len]);

        let rect = info.rcMonitor;
        monitors.push(MonitorInfo::new(
            name,
            rect.left,
            rect.top,
            rect.right - rect.left,
            rect.bottom - rect.top,
        ));

        TRUE
    }

    impl MonitorBackend for PlatformBackend {
        fn list_monitors() -> Result<Vec<MonitorInfo>, BackendError> {
            let mut monitors: Vec<MonitorInfo> = Vec::new();
            let ok = unsafe {
                EnumDisplayMonitors(
                    ptr::null_mut(),
                    ptr::null(),
                    Some(enum_monitor_proc),
                    &mut monitors as *mut Vec<MonitorInfo> as LPARAM,
                )
            };
            if ok == 0 {
                return Err(BackendError::new(
                    "Unable to enumerate monitors with EnumDisplayMonitors.",
                ));
            }
            if monitors.is_empty() {
                return Err(BackendError::new("No active displays found."));
            }
            Ok(monitors)
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
mod platform {
    use super::{BackendError, MonitorBackend, MonitorInfo};

    pub(super) struct PlatformBackend;

    impl MonitorBackend for PlatformBackend {
        fn list_monitors() -> Result<Vec<MonitorInfo>, BackendError> {
            Err(BackendError::new(format!(
                "Unsupported platform `{}`. No monitor backend is available.",
                std::env::consts::OS
            )))
        }
    }
}

use platform::PlatformBackend;

pub fn list_monitors() -> Result<Vec<MonitorInfo>, BackendError> {
    PlatformBackend::list_monitors()
}
