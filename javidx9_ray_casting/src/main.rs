#![allow(unused)]
use macroquad::prelude::*;
use std::cmp::{max, min};

struct Player {
    position: Vec2,
}

struct Map {
    size: IVec2,
    cell_size: IVec2,
    tiles: Vec<i32>,
}

struct Game {
    player: Player,
    map: Map,
    camera: Camera2D,
    intersection: Option<Vec2>,
}

impl Default for Game {
    fn default() -> Self {
        let render_target = render_target(512, 480);
        let mut camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: 0.0,
            w: 512.0,
            h: 480.0,
        });
        Game {
            player: Player {
                position: Vec2::ZERO,
            },
            map: Map {
                size: IVec2::new(32, 30),
                cell_size: IVec2::splat(16),
                tiles: vec![0; (32 * 30) as usize],
            },
            camera: Camera2D::from_display_rect(Rect {
                x: 0.0,
                y: 0.0,
                w: 512.0,
                h: 480.0,
            }),
            intersection: None,
        }
    }
}

impl Game {
    fn update(&mut self) -> bool {
        let delta = get_frame_time();

        if is_key_down(KeyCode::Escape) {
            return false;
        }

        let (mouse_x, mouse_y) = mouse_position();
        let world_mouse = self.camera.screen_to_world(Vec2::new(mouse_x, mouse_y));
        let mouse_cell = world_mouse / self.map.cell_size.as_f32();
        let cell = {
            let cell_x = (world_mouse.x / self.map.cell_size.x as f32) as i32;
            let cell_y = (world_mouse.y / self.map.cell_size.y as f32) as i32;
            IVec2::new(cell_x, cell_y)
        };

        if is_mouse_button_down(MouseButton::Right) {
            if (cell.x >= 0 && cell.x < self.map.size.x && cell.y >= 0 && cell.y < self.map.size.y)
            {
                let idx = (cell.y * self.map.size.x + cell.x) as usize;
                self.map.tiles[idx] = 1;
            }
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.player.position.x += 25.0 * delta;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.player.position.x -= 25.0 * delta;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.player.position.y += 25.0 * delta;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.player.position.y -= 25.0 * delta;
        }

        let ray_start = self.player.position;
        let ray_dir = (mouse_cell - ray_start).normalize();

        let a = (1.0 + (ray_dir.y / ray_dir.x) * (ray_dir.y / ray_dir.x)).sqrt();
        let b = (1.0 + (ray_dir.x / ray_dir.y) * (ray_dir.x / ray_dir.y)).sqrt();
        let ray_unit_step_size = Vec2::new(a, b);
        let mut map_check = ray_start.as_i32();
        let mut ray_length_1d = Vec2::ZERO;
        let mut step = IVec2::ZERO;

        if ray_dir.x < 0.0 {
            step.x = -1;
            ray_length_1d.x = (ray_start.x - map_check.x as f32) * ray_unit_step_size.x;
        } else {
            step.x = 1;
            ray_length_1d.x =
                ((map_check.x + 1) as f32 - ray_start.x) * ray_unit_step_size.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1;
            ray_length_1d.y = (ray_start.y - map_check.y as f32) * ray_unit_step_size.y;
        } else {
            step.y = 1;
            ray_length_1d.y =
                ((map_check.y + 1) as f32 - ray_start.y) * ray_unit_step_size.y;
        }

        let mut tile_found = false;
        let max_distance = 100.0;
        let mut distance = 0.0;

        while !tile_found && distance < max_distance {
            if ray_length_1d.x < ray_length_1d.y {
                distance = ray_length_1d.x;
                map_check.x += step.x;
                ray_length_1d.x += ray_unit_step_size.x;
            } else {
                distance = ray_length_1d.y;
                map_check.y += step.y;
                ray_length_1d.y += ray_unit_step_size.y;
            }

            if map_check.x >= 0
                && map_check.x < self.map.size.x
                && map_check.y >= 0
                && map_check.y < self.map.size.y
            {
                if self.map.tiles[(map_check.y * self.map.size.x + map_check.x) as usize] == 1 {
                    tile_found = true;
                }
            }
        }

        if tile_found {
            let ray_start = Vec2::new(ray_start.x as f32, ray_start.y as f32);
            self.intersection = Some(ray_start + ray_dir * distance);
        } else {
            self.intersection = None;
        }
        return true;
    }

    fn render(&mut self) {
        clear_background(Color::new(0.0, 0.0, 0.0, 1.0));

        let aspect = screen_width() / screen_height();
        let zoom = 512.0;

        self.camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: 0.0,
            w: screen_width(),
            h: screen_height(),
        });

        set_camera(&self.camera);

        for y in 0..self.map.size.y {
            for x in 0..self.map.size.x {
                let cell = self.map.tiles[(y * self.map.size.x + x) as usize];
                if (cell == 1) {
                    draw_rectangle(
                        (x * self.map.cell_size.x) as f32,
                        (y * self.map.cell_size.y) as f32,
                        self.map.cell_size.x as f32,
                        self.map.cell_size.y as f32,
                        BLUE,
                    );
                }
                draw_rectangle_lines(
                    (x * self.map.cell_size.x) as f32,
                    (y * self.map.cell_size.y) as f32,
                    self.map.cell_size.x as f32,
                    self.map.cell_size.y as f32,
                    1.0,
                    DARKGRAY,
                );
            }
        }

        let (mouse_x, mouse_y) = mouse_position();
        let world_mouse = self.camera.screen_to_world(Vec2::new(mouse_x, mouse_y));

        let (player_x, player_y) = (
            self.player.position.x * self.map.cell_size.x as f32,
            self.player.position.y * self.map.cell_size.y as f32,
        );
        if is_mouse_button_down(MouseButton::Left) {
            draw_line(player_x, player_y, world_mouse.x, world_mouse.y, 1.0, WHITE);

            if let Some(intersection) = self.intersection {
                draw_circle_lines(
                    intersection.x * self.map.cell_size.x as f32,
                    intersection.y * self.map.cell_size.y as f32,
                    4.0,
                    1.0,
                    WHITE,
                )
            }
        }

        draw_circle(player_x, player_y, 4.0, RED);
        draw_circle(world_mouse.x, world_mouse.y, 4.0, GREEN);
    }

    async fn run(&mut self) {
        'game: loop {
            if !self.update() {
                break 'game;
            }
            self.render();

            // draw_text(&format!("FPS: {}", get_fps()), 20.0, screen_height() - 40.0, 30.0, YELLOW);
            // draw_text(&format!("Frame Time: {}", get_frame_time()), 20.0, screen_height() - 20.0, 30.0, YELLOW);

            next_frame().await
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Ray casting".to_owned(),
        window_width: 512,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::default();

    game.run().await;
}
