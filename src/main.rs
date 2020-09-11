mod grid;
mod algorithm;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseButton;
use sdl2::video::Window;

use crate::grid::{Grid, Cell};
use crate::algorithm::{GameOfLife, Algorithm};

struct Main {
    grid: Grid,
    cell_size: u32,
    sdl_context: Sdl,
    canvas: WindowCanvas,
    algorithm: Box<dyn Algorithm>,
    running_alg: bool,
    fps: u32
}

impl Main {
    fn new(cell_size: u32, grid_w: u32, grid_h: u32) -> Main {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Game of Life", grid_w * cell_size, grid_h * cell_size)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::BLUE);
        canvas.clear();
        canvas.present();

        let mut grid = Grid::new(grid_w, grid_h);

        let temp_grid = Grid::new(grid_w, grid_h);
        let algorithm = GameOfLife::new(temp_grid);

        Main {
            grid,
            cell_size,
            sdl_context,
            canvas,
            algorithm: Box::new(algorithm),
            running_alg: false,
            fps: 10
        }
    }

    fn update(&mut self) {
        if self.running_alg {
            self.algorithm.update(&mut self.grid)
        }

        let mut window = self.canvas.window_mut();
        window.set_title(&*format!("Game of Life ({}running <space>) [{} fps <up/down>]", if self.running_alg { "" } else { "not " }, self.fps));
    }

    fn left_click(&mut self, x: u32, y: u32) {
        self.grid[(x, y)].on = true;
    }

    fn right_click(&mut self, x: u32, y: u32) {
        self.grid[(x, y)].on = false;
    }

    fn draw(&mut self) {
        let mut rects_on: Vec<Rect> = vec![];
        let mut rects_off: Vec<Rect> = vec![];

        for x in 0..self.grid.width() {
            for y in 0..self.grid.height() {
                let cell = &self.grid[(x, y)];

                fn get_rect(x: u32, y: u32, size: u32) -> Rect {
                    Rect::new((x * size) as i32, (y * size) as i32, size, size)
                }

                match cell.on {
                    true => {
                        rects_on.push(get_rect(x, y, self.cell_size));
                    }
                    false => {
                        rects_off.push(get_rect(x, y, self.cell_size));
                    }
                }
            }
        }

        self.canvas.set_draw_color(Color::RED);
        self.canvas.fill_rects(&*rects_on).expect("Rect draw failed");
        self.canvas.set_draw_color(Color::GRAY);
        self.canvas.fill_rects(&*rects_off).expect("Rect draw failed");
    }

    fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        self.running_alg = !self.running_alg;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        self.fps += 1;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        self.fps -= 1;
                        if self.fps < 1 {
                            self.fps = 1;
                        }
                    },
                    Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                        let grid_x: u32 = (x as u32) / self.cell_size;
                        let grid_y: u32 = (y as u32) / self.cell_size;

                        if grid_x < self.grid.width() && grid_y < self.grid.height() {
                            match mouse_btn {
                                MouseButton::Left => { self.left_click(grid_x, grid_y) },
                                MouseButton::Right => { self.right_click(grid_x, grid_y) },
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }

            self.update();
            self.draw();

            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.fps));
        }
    }
}

fn main() {
    let mut main: Main = Main::new(30, 60, 30);
    main.run();
}