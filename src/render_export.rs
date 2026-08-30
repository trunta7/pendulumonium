use minifb::{Key, Window, WindowOptions};
use crate::rk8solver::{PendulumParams, State};
use rtrb::{Consumer, PopError};
use std::sync::Arc;
use std::thread;

#[derive(Clone, Copy)]
pub struct RenderExportConfig {
	pub n: usize, // number of pendulums to render from the selection
    pub window_width: usize, // width of render window
    pub window_height: usize, // height of render window
    pub pixel_scale: f64, // pixels per unit of length
}

impl Default for  RenderExportConfig {
    fn default() -> Self {
        Self {
            n: 10,
            window_width: 800,
            window_height: 800,
            pixel_scale: 100.0
        }
    }
}

pub fn render_export(
    mut consumer: Consumer<Arc<Vec<State>>>,
    params: PendulumParams,
    config: RenderExportConfig,
    stop_flag: Arc<std::sync::atomic::AtomicBool>,
) {

    let n = config.n;

    let mut window = Window::new(
        "Double Pendulum Render - Press ESC to exit",
        config.window_width,
        config.window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    let mut buffer: Vec<u32> = vec![0; config.window_width * config.window_height];

    // scale pendulum to provided lengths
    let l1 = params.length1 * config.pixel_scale;
    let l2 = params.length2 * config.pixel_scale;

    let cx = config.window_width as f64 / 2.0;
    let cy = config.window_height as f64 / 3.0;

    loop {
        if stop_flag.load(std::sync::atomic::Ordering::Relaxed) || !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }

        match consumer.pop() {
            Ok(frame) => {
                let total_len = frame.len();
                if total_len == 0 {
                    continue;
                }

                buffer.fill(0x111111);
                let step = if n >= total_len { 1 } else { total_len / n };

                for i in 0..n {
                    let idx = (i * step).min(total_len - 1);
                    let state = frame[idx];

                    let x1 = cx + l1 * state.theta1.sin();
                    let y1 = cy + l1 * state.theta1.cos();
                    let x2 = x1 + l2 * state.theta2.sin();
                    let y2 = y1 + l2 * state.theta2.cos();

                    draw_line(&mut buffer, config.window_width, config.window_height, cx as i32, cy as i32, x1 as i32, y1 as i32, 0x0033bb55);
                    draw_line(&mut buffer, config.window_width, config.window_height, x1 as i32, y1 as i32, x2 as i32, y2 as i32, 0x00bb3333);
                    draw_pixel(&mut buffer, config.window_width, config.window_height, x2 as i32, y2 as i32, 0x00ffffff);
                }

                window.update_with_buffer(&buffer, config.window_width, config.window_height).unwrap();
            }
            Err(PopError::Empty) => {
                if consumer.is_abandoned() {
                    break;
                }
                thread::yield_now();
            }
        }
    }
}

fn draw_line(buffer: &mut [u32], width: usize, height: usize, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && (x as usize) < width && y >= 0 && (y as usize) < height {
            buffer[(y as usize) * width + (x as usize)] = color;
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn draw_pixel(buffer: &mut [u32], width: usize, height: usize, x: i32, y: i32, color: u32) {
    if x >= 0 && (x as usize) < width && y >= 0 && (y as usize) < height {
        buffer[(y as usize) * width + (x as usize)] = color;
    }
}