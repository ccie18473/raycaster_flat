use crate::prelude::*;

pub struct Game {
    pub player: Player,
    pub map: Map,
    pub canvas: Canvas,
}

impl Game {
    pub fn new(ctx: &mut Context, width: i32, height: i32) -> Game {
        Game {
            player: Player::new(),
            map: Map::new(),
            canvas: Canvas::new(ctx, width, height),
        }
    }
    pub fn new_size(&mut self, ctx: &mut Context, width: i32, height: i32) {
        self.canvas = Canvas::new(ctx, width, height);
    }
    pub fn update(&mut self, delta: f64) {
        self.player.update(&mut self.map, delta);
    }
}
