#![allow(unused)]

use macroquad::prelude::*;
use std::f32::consts::PI;

static DR: f32 = 0.0174533; // One deg in rad
const P2: f32 = PI / 2.0;
const P3: f32 = 3.0 * PI / 2.0;

const MAP_X: i32 = 8;
const MAP_Y: i32 = 8;
const MAP_SIZE: i32 = 64;
const MAP: [i32; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1, //
    1, 0, 1, 0, 0, 0, 0, 1, //
    1, 0, 1, 0, 0, 0, 0, 1, //
    1, 0, 1, 0, 0, 0, 0, 1, //
    1, 0, 0, 0, 0, 0, 0, 1, //
    1, 0, 0, 0, 0, 1, 0, 1, //
    1, 0, 0, 0, 0, 0, 0, 1, //
    1, 1, 1, 1, 1, 1, 1, 1, //
];

struct Player {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    a: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            x: 0.0,
            y: 0.0,
            dx: (0.0 as f32).cos() * 5.0,
            dy: (0.0 as f32).sin() * 5.0,
            a: 0.0,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Raycaster".to_owned(),
        window_width: 1024,
        window_height: 512,
        ..Default::default()
    }
}

fn draw_map2d() {
    for y in 0..MAP_Y {
        for x in 0..MAP_X {
            let color = if MAP[(y * MAP_Y + x) as usize] == 1 {
                WHITE
            } else {
                BLACK
            };
            draw_rectangle(
                (x * MAP_SIZE) as f32 + 1.0,
                (y * MAP_SIZE) as f32 + 1.0,
                MAP_SIZE as f32 - 2.0,
                MAP_SIZE as f32 - 2.0,
                color,
            )
        }
    }
}

fn draw_player(p: &Player) {
    draw_rectangle(p.x, p.y, 8.0, 8.0, YELLOW);
    draw_line(
        p.x + 4.0,
        p.y + 4.0,
        p.x + 4.0 + p.dx * 5.0,
        p.y + 4.0 + p.dy * 5.0,
        3.0,
        YELLOW,
    );
}

fn draw_rays2d(p: &Player) {
    let (player_x, player_y, player_angle) = (p.x + 4.0, p.y + 4.0, p.a);
    let (mut r, mut map_x, mut map_y, mut map_point, mut depth_of_field): (
        i32,
        i32,
        i32,
        i32,
        i32,
    ) = (0, 0, 0, 0, 0);
    let (mut ray_x, mut ray_y, mut ray_angle, mut ray_x_offset, mut ray_y_offset): (
        f32,
        f32,
        f32,
        f32,
        f32,
    ) = (0.0, 0.0, 0.0, 0.0, 0.0);
    ray_angle = player_angle - DR * 30.0;
    if ray_angle < 0.0 {
        ray_angle += 2.0 * PI;
    }
    if ray_angle > 2.0 * PI {
        ray_angle -= 2.0 * PI;
    }

    for r in 0..60 {
        depth_of_field = 0;
        let (mut dist_h, mut hx, mut hy): (f32, f32, f32) = (1000000.0, player_x, player_y);
        let a_tan = -1.0 / ray_angle.tan();
        if ray_angle > PI {
            // looking up
            ray_y = (((player_y as i32 >> 6) << 6) as f32 - 0.0001);
            ray_x = (player_y - ray_y) * a_tan + player_x;
            ray_y_offset = -64.0;
            ray_x_offset = -ray_y_offset * a_tan;
        }
        if ray_angle < PI {
            // looking down
            ray_y = (((player_y as i32 >> 6) << 6) as f32 + 64.0);
            ray_x = (player_y - ray_y) * a_tan + player_x;
            ray_y_offset = 64.0;
            ray_x_offset = -ray_y_offset * a_tan;
        }
        if ray_angle == 0.0 || ray_angle == PI {
            ray_x = player_x;
            ray_y = player_y;
            depth_of_field = 8;
        }

        while depth_of_field < 8 {
            map_x = ray_x as i32 >> 6;
            map_y = ray_y as i32 >> 6;
            map_point = map_y * MAP_X + map_x;

            if map_point > 0 && map_point < (MAP_X * MAP_Y) && MAP[map_point as usize] == 1 {
                hx = ray_x;
                hy = ray_y;
                dist_h = dist(player_x, player_y, hx, hy, ray_angle);
                depth_of_field = 8; // hit wall
            } else {
                ray_x += ray_x_offset;
                ray_y += ray_y_offset;
                depth_of_field += 1;
            }
        }

        depth_of_field = 0;
        let (mut dist_v, mut vx, mut vy): (f32, f32, f32) = (1000000.0, player_x, player_y);
        let n_tan = -ray_angle.tan();
        if ray_angle > P2 && ray_angle < P3 {
            // looking left
            ray_x = (((player_x as i32 >> 6) << 6) as f32 - 0.0001);
            ray_y = (player_x - ray_x) * n_tan + player_y;
            ray_x_offset = -MAP_SIZE as f32;
            ray_y_offset = -ray_x_offset * n_tan;
        }
        if ray_angle < P2 || ray_angle > P3 {
            // looking right
            ray_x = (((player_x as i32 >> 6) << 6) as f32 + MAP_SIZE as f32);
            ray_y = (player_x - ray_x) * n_tan + player_y;
            ray_x_offset = MAP_SIZE as f32;
            ray_y_offset = -ray_x_offset * n_tan;
        }
        if ray_angle == 0.0 || ray_angle == PI {
            ray_x = player_x;
            ray_y = player_y;
            depth_of_field = 8;
        }

        while depth_of_field < 8 {
            map_x = ray_x as i32 >> 6;
            map_y = ray_y as i32 >> 6;
            map_point = map_y * MAP_X + map_x;

            if map_point > 0 && map_point < (MAP_X * MAP_Y) && MAP[map_point as usize] == 1 {
                vx = ray_x;
                vy = ray_y;
                dist_v = dist(player_x, player_y, vx, vy, ray_angle);
                depth_of_field = 8; // hit wall
            } else {
                ray_x += ray_x_offset;
                ray_y += ray_y_offset;
                depth_of_field += 1;
            }
        }

        if (dist_v < dist_h) {
            ray_x = vx;
            ray_y = vy;
        }
        if (dist_h < dist_v) {
            ray_x = hx;
            ray_y = hy;
        }

        draw_line(player_x, player_y, ray_x, ray_y, 3.0, RED);
        ray_angle += DR;
        if ray_angle < 0.0 {
            ray_angle += 2.0 * PI;
        }
        if ray_angle > 2.0 * PI {
            ray_angle -= 2.0 * PI;
        }
    }
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32, ang: f32) -> f32 {
    ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt()
}

fn update(p: &mut Player) {
    let angle_delta_scale = 2.0;

    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        p.a -= 0.1 / angle_delta_scale;
        if p.a < 0.0 {
            p.a += 2.0 * PI;
        }
        p.dx = p.a.cos() * 5.0;
        p.dy = p.a.sin() * 5.0;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        p.a += 0.1 / angle_delta_scale;
        if p.a > 2.0 * PI {
            p.a -= 2.0 * PI;
        }
        p.dx = p.a.cos() * 5.0;
        p.dy = p.a.sin() * 5.0;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        p.x += p.dx;
        p.y += p.dy;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        p.x -= p.dx;
        p.y -= p.dy;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut p = Player {
        x: 300.0,
        y: 300.0,
        ..Default::default()
    };

    'game: loop {
        if is_key_down(KeyCode::Escape) {
            break 'game;
        }
        clear_background(Color::new(0.3, 0.3, 0.3, 1.0));

        update(&mut p);
        draw_map2d();
        draw_rays2d(&p);
        draw_player(&p);

        next_frame().await
    }
}
