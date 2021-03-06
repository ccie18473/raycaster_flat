use crate::prelude::*;

type Point2 = glam::Vec2;

pub struct Canvas {
    pub width: i32,
    pub height: i32,
}

impl Canvas {
    pub fn new(_ctx: &mut Context, width: i32, height: i32) -> Canvas {
        Canvas { width, height }
    }
    pub fn draw(&mut self, ctx: &mut Context, player: &mut Player, map: &mut Map) -> GameResult {
        let mut mesh_builder = ggez::graphics::MeshBuilder::new();
        for x in 0..self.width {
            let ray = crate::Ray::new(player, map, x, self.width, self.height);

            mesh_builder.line(
                &[
                    Point2::new(x as f32, ray.draw_start as f32),
                    Point2::new(x as f32, ray.draw_end as f32),
                ],
                1.0,
                ray.color,
            )?;
        }

        let mesh = mesh_builder.build(ctx)?;

        ggez::graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::default())?;

        Ok(())
    }
    pub fn draw_fps(&mut self, ctx: &mut Context) -> GameResult {
        let fps = ggez::timer::fps(ctx);
        let fps_display = ggez::graphics::Text::new(format!("FPS: {:.2}", fps));
        let p = cgmath::Point2::new(0.0, 0.0);
        ggez::graphics::draw(ctx, &fps_display, (p,))?;

        Ok(())
    }
}
