extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{SystemTime, UNIX_EPOCH};

const RADIUS: i32 = 250;
const CENTER_X: i32 = 400;
const CENTER_Y: i32 = 300;

type PointType = rect::Point;

fn get_time() -> (f64, f64, f64) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let mut total_seconds = now.as_secs_f64();
    let offset = 5.0 * 3600.0 + 30.0 * 60.0;
    total_seconds += offset;

    let seconds = total_seconds % 60.0;
    let minutes = (total_seconds / 60.0) % 60.0;
    let hour = (total_seconds / 3600.0) % 12.0;

    (seconds, minutes, hour)
}

fn draw_hand(canvas: &mut Canvas<Window>, angle: f64, length_factor: f64, color: Color) {
    let rad = angle.to_radians();

    let r = RADIUS as f64 * length_factor;

    let x = CENTER_X as f64 + r * rad.sin();
    let y = CENTER_Y as f64 - r * rad.cos();

    let start = PointType::new(CENTER_X, CENTER_Y);
    let end = PointType::new(x as i32, y as i32);

    canvas.set_draw_color(color);
    let _ = canvas.draw_line(start, end);
}

fn draw_hands(canvas: &mut Canvas<Window>) {
    let (s, m, h) = get_time();

    let sec_angle = s * 6.0;
    let min_angle = m * 6.0;
    let hour_angle = h * 30.0;

    draw_hand(canvas, sec_angle, 0.9, Color::RGB(255, 0, 0));
    draw_hand(canvas, min_angle, 0.75, Color::RGB(0, 200, 255));
    draw_hand(canvas, hour_angle, 0.55, Color::RGB(255, 255, 255));
}

fn draw_circle(canvas: &mut Canvas<Window>, centre_x: i32, centre_y: i32, radius: i32) {
    let diameter = radius * 2;

    let mut x = radius - 1;
    let mut y = 0;
    let mut tx = 1;
    let mut ty = 1;
    let mut error = tx - diameter;
    let mut points: Vec<PointType> = Vec::new();

    while x >= y {
        points.push(PointType::new(centre_x + x, centre_y - y));
        points.push(PointType::new(centre_x + x, centre_y + y));
        points.push(PointType::new(centre_x - x, centre_y - y));
        points.push(PointType::new(centre_x - x, centre_y + y));
        points.push(PointType::new(centre_x + y, centre_y - x));
        points.push(PointType::new(centre_x + y, centre_y + x));
        points.push(PointType::new(centre_x - y, centre_y - x));
        points.push(PointType::new(centre_x - y, centre_y + x));

        if error <= 0 {
            y += 1;
            error += ty;
            ty += 2;
        }

        if error > 0 {
            x -= 1;
            tx += 2;
            error += tx - diameter;
        }
    }
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_points(&*points).unwrap();
}

fn draw_clock_lines(canvas: &mut Canvas<Window>) {
    let mut angle_in_deg: f64 = 0.0;
    let mut length_factor;

    while angle_in_deg < 360.0 {
        let rad = angle_in_deg.to_radians();

        if angle_in_deg % 30.0 == 0.0 {
            length_factor = 0.85;
        } else {
            length_factor = 0.9;
        }

        let x1 = CENTER_X as f64 + (RADIUS as f64 * length_factor) * rad.sin();
        let x2 = CENTER_X as f64 + (RADIUS as f64 * 0.95) * rad.sin();

        let y1 = CENTER_Y as f64 - (RADIUS as f64 * length_factor) * rad.cos();
        let y2 = CENTER_Y as f64 - (RADIUS as f64 * 0.95) * rad.cos();

        let point1 = PointType::new(x1 as i32, y1 as i32);
        let point2 = PointType::new(x2 as i32, y2 as i32);

        canvas.set_draw_color(Color::RGB(100, 255, 250));
        let _ = canvas.draw_line(point1, point2);

        angle_in_deg += 6.0;
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw_circle(&mut canvas, CENTER_X, CENTER_Y, RADIUS);
        draw_clock_lines(&mut canvas);
        draw_hands(&mut canvas);

        canvas.present();
    }
}
