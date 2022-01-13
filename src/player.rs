use crate::prelude::*;

const LEFT_KEY: KeyCode = ggez::event::KeyCode::Left;
const RIGHT_KEY: KeyCode = ggez::event::KeyCode::Right;
const UP_KEY: KeyCode = ggez::event::KeyCode::Up;
const DOWN_KEY: KeyCode = ggez::event::KeyCode::Down;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    None = 0,
    LookLeft,
    LookRight,
    MoveForward,
    MoveBackward,
}

pub struct Player {
    pub pos_x: f64,
    pub pos_y: f64,
    pub dir_x: f64,
    pub dir_y: f64,
    pub plane_x: f64,
    pub plane_y: f64,

    pub action: Action,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos_x: 22.0, //x and y start position
            pos_y: 12.0,
            dir_x: -1.0, //initial direction vector
            dir_y: 0.0,
            plane_x: 0.0, //the 2d raycaster version of camera plane
            plane_y: 0.66,

            action: Action::None,
        }
    }
    pub fn handle_inputs(&mut self, keycode: KeyCode, pressed: bool) {
        if keycode == UP_KEY {
            if pressed {
                if self.action == Action::None {
                    self.action = Action::MoveForward;
                }
            } else {
                self.action = Action::None;
            }
        } else if keycode == DOWN_KEY {
            if pressed {
                if self.action == Action::None {
                    self.action = Action::MoveBackward;
                }
            } else {
                self.action = Action::None;
            }
        } else if keycode == LEFT_KEY {
            if pressed {
                if self.action == Action::None {
                    self.action = Action::LookLeft;
                }
            } else {
                self.action = Action::None;
            }
        } else if keycode == RIGHT_KEY {
            if pressed {
                if self.action == Action::None {
                    self.action = Action::LookRight;
                }
            } else {
                self.action = Action::None;
            }
        }
    }

    pub fn update(&mut self, map: &mut Map, delta: f64) {
        //speed modifiers
        let move_speed = 2.0 * delta;
        let rot_speed = 1.0 * delta;
        //move forward if no wall in front of you
        if self.action == Action::MoveForward {
            if map.table[(self.pos_x + self.dir_x * move_speed) as usize][(self.pos_y) as usize]
                == 0
            {
                self.pos_x += self.dir_x * move_speed;
            }
            if map.table[self.pos_x as usize][(self.pos_y + self.dir_y * move_speed) as usize] == 0
            {
                self.pos_y += self.dir_y * move_speed;
            }
        //move backwards if no wall behind you
        } else if self.action == Action::MoveBackward {
            if map.table[(self.pos_x - self.dir_x * move_speed) as usize][(self.pos_y) as usize]
                == 0
            {
                self.pos_x -= self.dir_x * move_speed;
            }
            if map.table[self.pos_x as usize][(self.pos_y - self.dir_y * move_speed) as usize] == 0
            {
                self.pos_y -= self.dir_y * move_speed;
            }
        //rotate to the left
        } else if self.action == Action::LookLeft {
            //both camera direction and camera plane must be rotated
            let old_dir_x = self.dir_x;
            self.dir_x = self.dir_x * (rot_speed).cos() - self.dir_y * (rot_speed).sin();
            self.dir_y = old_dir_x * (rot_speed).sin() + self.dir_y * (rot_speed).cos();

            let old_plane_x = self.plane_x;
            self.plane_x = self.plane_x * (rot_speed).cos() - self.plane_y * (rot_speed).sin();
            self.plane_y = old_plane_x * (rot_speed).sin() + self.plane_y * (rot_speed).cos();
        //rotate to the right
        } else if self.action == Action::LookRight {
            //both camera direction and camera plane must be rotated
            let old_dir_x = self.dir_x;
            self.dir_x = self.dir_x * (-rot_speed).cos() - self.dir_y * (-rot_speed).sin();
            self.dir_y = old_dir_x * (-rot_speed).sin() + self.dir_y * (-rot_speed).cos();

            let old_plane_x = self.plane_x;
            self.plane_x = self.plane_x * (-rot_speed).cos() - self.plane_y * (-rot_speed).sin();
            self.plane_y = old_plane_x * (-rot_speed).sin() + self.plane_y * (-rot_speed).cos();
        } else {
            self.action = Action::None;
        }
    }
}
