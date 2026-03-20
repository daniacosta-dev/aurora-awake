use std::{thread, time::Duration};

use enigo::{Coordinate, Enigo, Mouse, Settings};

use crate::models::MovementPattern;

const FRAME_MS: u64 = 16; // ~60 FPS

pub fn jiggle_cursor(
    pixels: i32,
    duration_ms: u64,
    pattern: MovementPattern,
) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo =
        Enigo::new(&settings).map_err(|e| format!("Failed to initialize Enigo: {e}"))?;

    match pattern {
        MovementPattern::Line => animate_line(&mut enigo, pixels, duration_ms)?,
        MovementPattern::Square => animate_square(&mut enigo, pixels, duration_ms)?,
        MovementPattern::Circle => animate_circle(&mut enigo, pixels, duration_ms)?,
        MovementPattern::ZigZag => animate_zigzag(&mut enigo, pixels, duration_ms)?,
    }

    Ok(())
}

fn sleep_frame() {
    thread::sleep(Duration::from_millis(FRAME_MS));
}

fn move_rel(enigo: &mut Enigo, dx: i32, dy: i32) -> Result<(), String> {
    enigo
        .move_mouse(dx, dy, Coordinate::Rel)
        .map_err(|e| format!("Mouse move failed: {e}"))
}

fn animate_points(
    enigo: &mut Enigo,
    points: &[(f64, f64)],
    duration_ms: u64,
) -> Result<(), String> {
    let frames = (duration_ms / FRAME_MS).max(points.len() as u64).max(1) as usize;

    let mut prev_x = points[0].0;
    let mut prev_y = points[0].1;

    let mut moved_x = 0i32;
    let mut moved_y = 0i32;

    for frame in 1..=frames {
        let t = frame as f64 / frames as f64;
        let idx_f = t * (points.len() as f64 - 1.0);
        let idx = idx_f.floor() as usize;
        let next_idx = (idx + 1).min(points.len() - 1);
        let local_t = idx_f - idx as f64;

        let x = points[idx].0 + (points[next_idx].0 - points[idx].0) * local_t;
        let y = points[idx].1 + (points[next_idx].1 - points[idx].1) * local_t;

        let dx_f = x - prev_x;
        let dy_f = y - prev_y;

        // acumulamos fracciones para no perder movimiento por redondeo
        let dx = dx_f.round() as i32;
        let dy = dy_f.round() as i32;

        if dx != 0 || dy != 0 {
            move_rel(enigo, dx, dy)?;
            moved_x += dx;
            moved_y += dy;
        }

        prev_x = x;
        prev_y = y;

        sleep_frame();
    }

    // volver exactamente al origen
    if moved_x != 0 || moved_y != 0 {
        move_rel(enigo, -moved_x, -moved_y)?;
    }

    Ok(())
}

fn animate_line(enigo: &mut Enigo, pixels: i32, duration_ms: u64) -> Result<(), String> {
    let p = pixels as f64;
    let points = vec![(0.0, 0.0), (p, 0.0), (0.0, 0.0)];
    animate_points(enigo, &points, duration_ms)
}

fn animate_square(enigo: &mut Enigo, pixels: i32, duration_ms: u64) -> Result<(), String> {
    let p = pixels as f64;
    let points = vec![
        (0.0, 0.0),
        (p, 0.0),
        (p, p),
        (0.0, p),
        (0.0, 0.0),
    ];
    animate_points(enigo, &points, duration_ms)
}

fn animate_zigzag(enigo: &mut Enigo, pixels: i32, duration_ms: u64) -> Result<(), String> {
    let p = pixels as f64;
    let points = vec![
        (0.0, 0.0),
        (p, p),
        (2.0 * p, 0.0),
        (p, -p),
        (0.0, 0.0),
    ];
    animate_points(enigo, &points, duration_ms)
}

fn animate_circle(enigo: &mut Enigo, radius: i32, duration_ms: u64) -> Result<(), String> {
    let steps = 60usize;
    let r = radius as f64;

    let mut points = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let angle = (i as f64 / steps as f64) * std::f64::consts::TAU;
        points.push((r * angle.cos(), r * angle.sin()));
    }

    animate_points(enigo, &points, duration_ms)
}