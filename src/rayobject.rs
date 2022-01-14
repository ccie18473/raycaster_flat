use crate::prelude::*;

pub struct Ray {
    pub draw_start: i32,
    pub draw_end: i32,
    pub color: Color,
}
impl Ray {
    pub fn new(player: &Player, map: &Map, x: i32, width: i32, height: i32) -> Ray {
        //calculate ray position and direction
        let camera_x = (2 * x) as f64 / (width as f64) - 1.0;
        let ray_dir_x = player.dir_x + player.plane_x * camera_x;
        let ray_dir_y = player.dir_y + player.plane_y * camera_x;
        //which box of the map we're in
        let mut map_x = player.pos_x as isize;
        let mut map_y = player.pos_y as isize;
        //length of ray from current position to next x or y-side
        let mut side_dist_x: f64;
        let mut side_dist_y: f64;
        //length of ray from one x or y-side to next x or y-side
        //these are derived as:
        //deltaDistX = sqrt(1 + (rayDirY * rayDirY) / (rayDirX * rayDirX))
        //deltaDistY = sqrt(1 + (rayDirX * rayDirX) / (rayDirY * rayDirY))
        //which can be simplified to abs(|rayDir| / rayDirX) and abs(|rayDir| / rayDirY)
        //where |rayDir| is the length of the vector (rayDirX, rayDirY). Its length,
        //unlike (dirX, dirY) is not 1, however this does not matter, only the
        //ratio between deltaDistX and deltaDistY matters, due to the way the DDA
        //stepping further below works. So the values can be computed as below.
        // Division through zero is prevented, even though technically that's not
        // needed in C++ with IEEE 754 floating point values.
        let delta_dist_x = if ray_dir_x == 0.0 {
            1e30
        } else {
            (1.0 / ray_dir_x).abs()
        };
        let delta_dist_y = if ray_dir_y == 0.0 {
            1e30
        } else {
            (1.0 / ray_dir_y).abs()
        };

        let perp_wall_dist: f64;
        //what direction to step in x or y-direction (either +1 or -1)
        let step_x: isize;
        let step_y: isize;

        let mut hit: usize = 0;
        let mut side: usize = 0;

        if ray_dir_x < 0.0 {
            step_x = -1;
            side_dist_x = (player.pos_x - map_x as f64) * delta_dist_x;
        } else {
            step_x = 1;
            side_dist_x = (map_x as f64 + 1.0 - player.pos_x) * delta_dist_x;
        }
        if ray_dir_y < 0.0 {
            step_y = -1;
            side_dist_y = (player.pos_y - map_y as f64) * delta_dist_y;
        } else {
            step_y = 1;
            side_dist_y = (map_y as f64 + 1.0 - player.pos_y) * delta_dist_y;
        }
        //perform DDA
        while hit == 0 {
            //jump to next map square, either in x-direction, or in y-direction
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }
            //Check if ray has hit a wall
            if map.table[map_x as usize][map_y as usize] > 0 {
                hit = 1;
            }
        }
        //Calculate distance projected on camera direction. This is the shortest distance from the point where the wall is
        //hit to the camera plane. Euclidean to center camera point would give fisheye effect!
        //This can be computed as (mapX - posX + (1 - stepX) / 2) / rayDirX for side == 0, or same formula with Y
        //for size == 1, but can be simplified to the code below thanks to how sideDist and deltaDist are computed:
        //because they were left scaled to |rayDir|. sideDist is the entire length of the ray above after the multiple
        //steps, but we subtract deltaDist once because one step more into the wall was taken above.
        if side == 0 {
            perp_wall_dist = side_dist_x - delta_dist_x;
        } else {
            perp_wall_dist = side_dist_y - delta_dist_y;
        }
        //Calculate height of line to draw on screen
        let line_height: i32 = (height as f64 / perp_wall_dist) as i32;
        //calculate lowest and highest pixel to fill in current stripe
        let mut draw_start: i32 = -line_height / 2 + height / 2;
        if draw_start < 0 {
            draw_start = 0;
        }
        let mut draw_end: i32 = line_height / 2 + height / 2;
        if draw_end >= height {
            draw_end = height - 1;
        }
        //choose wall color
        let mut color: Color;

        match map.table[map_x as usize][map_y as usize] {
            1 => {
                color = Color::RED;
            }
            2 => {
                color = Color::GREEN;
            }
            3 => {
                color = Color::BLUE;
            }
            4 => {
                color = Color::WHITE;
            }
            _ => {
                color = Color::YELLOW;
            }
        }
        //give x and y sides different brightness
        if side == 1 {
            color = Color {
                r: color.r / 2.0,
                g: color.g / 2.0,
                b: color.b / 2.0,
                a: 1.0,
            };
        }
        //draw the pixels of the stripe as a vertical line
        Ray {
            draw_start,
            draw_end,
            color,
        }
    }
}
