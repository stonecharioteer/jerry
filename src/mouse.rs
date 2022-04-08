use mouse_rs::Mouse;
use std::{thread, time::Duration};

use crate::geometry::Point;

pub fn animate_mouse(origin: &Point) -> Result<(), String> {
    let mouse_controller = Mouse::new();
    let spiral_points: Vec<Point> = origin.generate_spiral(None, None);
    for point in spiral_points {
        println!("Spiral: {:?}", point);
        _ = mouse_controller.move_to(point.x, point.y);
        thread::sleep(Duration::from_millis(100));
    }
    _ = mouse_controller.move_to(origin.x, origin.y);
    Ok(())
}
