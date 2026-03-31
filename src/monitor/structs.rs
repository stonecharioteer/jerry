use std::collections::HashMap;

use crate::geometry::{BoundingBox, Direction, Point, Rectangle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl MonitorInfo {
    pub fn new(name: impl Into<String>, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            name: name.into(),
            x,
            y,
            width: width.max(1),
            height: height.max(1),
        }
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    pub fn center_x(&self) -> i32 {
        self.left() + (self.width / 2)
    }

    pub fn center_y(&self) -> i32 {
        self.top() + (self.height / 2)
    }
}

impl Rectangle for MonitorInfo {
    fn contains_point(&self, x: i32, y: i32) -> bool {
        (x >= self.left()) && (x <= self.right()) && (y >= self.top()) && (y <= self.bottom())
    }

    fn top_left(&self) -> Point {
        Point {
            x: self.left(),
            y: self.top(),
        }
    }

    fn top_right(&self) -> Point {
        Point {
            x: self.right(),
            y: self.top(),
        }
    }

    fn bottom_left(&self) -> Point {
        Point {
            x: self.left(),
            y: self.bottom(),
        }
    }

    fn bottom_right(&self) -> Point {
        Point {
            x: self.right(),
            y: self.bottom(),
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self)
    }
}

pub type AdjacencyGraph = HashMap<(usize, Direction), usize>;

pub fn build_adjacency_graph(monitors: &[MonitorInfo], wrap_around: bool) -> AdjacencyGraph {
    let mut graph: AdjacencyGraph = HashMap::new();
    let directions = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];

    for (index, _) in monitors.iter().enumerate() {
        for direction in directions {
            if let Some(next_index) =
                select_next_monitor_index(monitors, index, direction, wrap_around)
            {
                graph.insert((index, direction), next_index);
            }
        }
    }

    graph
}

pub fn select_next_monitor_index(
    monitors: &[MonitorInfo],
    current_index: usize,
    direction: Direction,
    wrap_around: bool,
) -> Option<usize> {
    if current_index >= monitors.len() {
        return None;
    }

    let current = &monitors[current_index];
    let mut best: Option<(usize, (i32, i32, i32))> = None;

    for (index, candidate) in monitors.iter().enumerate() {
        if index == current_index {
            continue;
        }
        let Some(score) = forward_score(current, candidate, direction) else {
            continue;
        };

        match best {
            Some((_, best_score)) if score >= best_score => {}
            _ => best = Some((index, score)),
        }
    }

    if let Some((index, _)) = best {
        return Some(index);
    }

    if !wrap_around {
        return None;
    }

    let mut wrap_best: Option<(usize, (i32, i32, i32))> = None;
    for (index, candidate) in monitors.iter().enumerate() {
        if index == current_index {
            continue;
        }
        let score = wrap_score(current, candidate, direction);
        match wrap_best {
            Some((_, best_score)) if score >= best_score => {}
            _ => wrap_best = Some((index, score)),
        }
    }

    wrap_best.map(|(index, _)| index)
}

fn forward_score(
    current: &MonitorInfo,
    candidate: &MonitorInfo,
    direction: Direction,
) -> Option<(i32, i32, i32)> {
    match direction {
        Direction::Left => {
            if candidate.center_x() >= current.center_x() {
                return None;
            }
            Some((
                (current.left() - candidate.right()).max(0),
                axis_gap(
                    current.top(),
                    current.bottom(),
                    candidate.top(),
                    candidate.bottom(),
                ),
                (candidate.center_y() - current.center_y()).abs(),
            ))
        }
        Direction::Right => {
            if candidate.center_x() <= current.center_x() {
                return None;
            }
            Some((
                (candidate.left() - current.right()).max(0),
                axis_gap(
                    current.top(),
                    current.bottom(),
                    candidate.top(),
                    candidate.bottom(),
                ),
                (candidate.center_y() - current.center_y()).abs(),
            ))
        }
        Direction::Up => {
            if candidate.center_y() >= current.center_y() {
                return None;
            }
            Some((
                (current.top() - candidate.bottom()).max(0),
                axis_gap(
                    current.left(),
                    current.right(),
                    candidate.left(),
                    candidate.right(),
                ),
                (candidate.center_x() - current.center_x()).abs(),
            ))
        }
        Direction::Down => {
            if candidate.center_y() <= current.center_y() {
                return None;
            }
            Some((
                (candidate.top() - current.bottom()).max(0),
                axis_gap(
                    current.left(),
                    current.right(),
                    candidate.left(),
                    candidate.right(),
                ),
                (candidate.center_x() - current.center_x()).abs(),
            ))
        }
    }
}

fn wrap_score(
    current: &MonitorInfo,
    candidate: &MonitorInfo,
    direction: Direction,
) -> (i32, i32, i32) {
    match direction {
        Direction::Left => (
            -candidate.right(),
            axis_gap(
                current.top(),
                current.bottom(),
                candidate.top(),
                candidate.bottom(),
            ),
            (candidate.center_y() - current.center_y()).abs(),
        ),
        Direction::Right => (
            candidate.left(),
            axis_gap(
                current.top(),
                current.bottom(),
                candidate.top(),
                candidate.bottom(),
            ),
            (candidate.center_y() - current.center_y()).abs(),
        ),
        Direction::Up => (
            -candidate.bottom(),
            axis_gap(
                current.left(),
                current.right(),
                candidate.left(),
                candidate.right(),
            ),
            (candidate.center_x() - current.center_x()).abs(),
        ),
        Direction::Down => (
            candidate.top(),
            axis_gap(
                current.left(),
                current.right(),
                candidate.left(),
                candidate.right(),
            ),
            (candidate.center_x() - current.center_x()).abs(),
        ),
    }
}

fn axis_gap(a0: i32, a1: i32, b0: i32, b1: i32) -> i32 {
    let overlap = overlap_len(a0, a1, b0, b1);
    if overlap > 0 {
        return 0;
    }

    if b1 < a0 {
        a0 - b1
    } else if b0 > a1 {
        b0 - a1
    } else {
        0
    }
}

fn overlap_len(a0: i32, a1: i32, b0: i32, b1: i32) -> i32 {
    (a1.min(b1) - a0.max(b0)).max(0)
}

#[cfg(test)]
mod tests {
    use super::{select_next_monitor_index, MonitorInfo};
    use crate::geometry::Direction;

    fn monitors() -> Vec<MonitorInfo> {
        vec![
            MonitorInfo::new("left", -1920, 0, 1920, 1080),
            MonitorInfo::new("center", 0, 0, 1920, 1080),
            MonitorInfo::new("right", 2020, 150, 1920, 1080),
            MonitorInfo::new("up", 0, -1080, 1920, 1080),
            MonitorInfo::new("down", 0, 1080, 1920, 1080),
        ]
    }

    #[test]
    fn finds_nearest_monitor_with_nonzero_gap() {
        let displays = monitors();
        let current_index = displays.iter().position(|m| m.name == "center").unwrap();
        let next_index =
            select_next_monitor_index(&displays, current_index, Direction::Right, false).unwrap();
        assert_eq!(displays[next_index].name, "right");
    }

    #[test]
    fn wraps_when_enabled() {
        let displays = monitors();
        let current_index = displays.iter().position(|m| m.name == "right").unwrap();
        let next_index =
            select_next_monitor_index(&displays, current_index, Direction::Right, true).unwrap();
        assert_eq!(displays[next_index].name, "left");
    }

    #[test]
    fn returns_none_when_wrap_is_disabled_and_no_candidate_exists() {
        let displays = monitors();
        let current_index = displays.iter().position(|m| m.name == "right").unwrap();
        let next_index =
            select_next_monitor_index(&displays, current_index, Direction::Right, false);
        assert!(next_index.is_none());
    }
}
