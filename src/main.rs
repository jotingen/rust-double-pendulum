use std::f32::consts::PI;

use ggez::*;
use glam::*;
use rand::Rng;

const G:f32 = 6.674e-11;

#[derive(Clone, Copy, Debug)]
struct Pendulum {
    length: f32,
    angle: f32,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    color: graphics::Color,
}

impl Pendulum {
    fn new(ctx: &Context) -> Pendulum {
        let mut rng = rand::thread_rng();
        let length: f32 = rng.gen_range(50.0..100.0);
        let angle: f32 = rng.gen_range(0.0..2.0 * PI);
        let velocity: f32 = rng.gen_range(0.0..1.0);
        let mass: f32 = rng.gen_range(10.0..15.0);
        Pendulum {
            length,
            angle,
            velocity: vec2(velocity * (angle+PI/2.0).sin(), velocity * (angle+PI/2.0).cos()),
            acceleration: vec2(0.0, 0.0),
            mass,
            color: graphics::Color::WHITE,
        }
    }
    fn with_length(mut self, length: f32) -> Pendulum {
        self.length = length;
        self
    }
    fn with_angle(mut self, angle: f32) -> Pendulum {
        self.angle = angle;
        self
    }
    fn with_velocity(mut self, velocity: Vec2) -> Pendulum {
        self.velocity = velocity;
        self
    }
    fn with_mass(mut self, mass: f32) -> Pendulum {
        self.mass = mass;
        self
    }
    fn with_color(mut self, color: graphics::Color) -> Pendulum {
        self.color = color;
        self
    }
    fn calculate_force(&mut self, other: &Pendulum) {
        //self.acceleration = vec2(0.0, 0.0);
        //let r_squared = self.length.distance_squared(other.length);
        //let normal = -(self.length - other.length).normalize();
        //self.acceleration +=
        //    (((G * (self.mass) * (other.mass)) / r_squared)  * normal)
        //        / self.mass;
    }
    fn update(&mut self, dt: f32) {
        //self.velocity += self.acceleration * dt;
        //self.length += self.velocity * dt;
    }
    fn end_point(&self, start_point: Vec2) -> Vec2 {
        start_point + vec2(self.length * self.angle.sin(), self.length * self.angle.cos())
    }
    fn draw_line(self, ctx: &Context, start_point: Vec2) -> GameResult<graphics::Mesh> {
        graphics::Mesh::new_line(
            ctx,
            &[start_point, self.end_point(start_point)],
            1.0,
            graphics::Color::WHITE,
        )
    }
    fn draw_mass(self, ctx: &Context, start_point: Vec2) -> GameResult<graphics::Mesh> {
        graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.end_point(start_point),
            10.0,
            2.0,
            graphics::Color::WHITE,
        )
    }
}

struct State {
    dt: std::time::Duration,
    pendulums: Vec<Pendulum>,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let mut rng = rand::thread_rng();
        let mut pendulums: Vec<Pendulum> = vec![];
        for _ in 0..2 {
            pendulums.push(Pendulum::new(&ctx));
        }
        Ok(State {
            dt: ctx.time.delta(),
            pendulums,
        })
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        // //Determine forces
        // for i in 0..self.pendulums.len() {
        //     for j in 0..self.pendulums.len() {
        //         if i != j {
        //             let other = self.pendulums[j].clone();
        //             self.pendulums[i].calculate_force(&other);
        //         }
        //     }
        // }
        // //Update velocity/position
        // for i in 0..self.pendulums.len() {
        //     self.pendulums[i].update(10.0);
        // }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (width, height) = ctx.gfx.drawable_size();
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.2, 0.3, 1.0].into()),
        );

        // //Get min/max dimensions
        // let mut min_x = self.pendulums[0].length.x;
        // let mut max_x = self.pendulums[0].length.x;
        // let mut min_y = self.pendulums[0].length.y;
        // let mut max_y = self.pendulums[0].length.y;
        // for object in &self.pendulums {
        //     if object.length.x < min_x {
        //         min_x = object.length.x;
        //     }
        //     if object.length.x > max_x {
        //         max_x = object.length.x;
        //     }
        //     if object.length.y < min_y {
        //         min_y = object.length.y;
        //     }
        //     if object.length.y > max_y {
        //         max_y = object.length.y;
        //     }
        // }

        // //Space used
        // let full_x = (max_x - min_x) * 1.2;
        // let full_y = (max_y - min_x) * 1.2;

        // //Determine the scaling needed
        // let scale_x = width / full_x;
        // let scale_y = height / full_y;
        // let scale = if scale_y < scale_x { scale_y } else { scale_x };

        // //Generate offset for greater scale
        // //Adjust by range of objects to align to edge, and then by scaled distance to center
        // let width_scaled = (max_x - min_x) * scale;
        // let height_scaled = (max_y - min_y) * scale;
        // let width_offset_scaled = (width - width_scaled) / 2.0;
        // let height_offset_scaled = (height - height_scaled) / 2.0;
        // let width_offset = width_offset_scaled / scale;
        // let height_offset = height_offset_scaled / scale;
        // let offset = vec2(-min_x + width_offset, -min_y + height_offset);

        // //println!(
        // //    "dt = {}ns  {}x{} {:?}",
        // //    self.dt.as_nanos(),
        // //    width,
        // //    height,
        // //    self.objects[0],
        // //);

        let mut start_point: Vec2 = vec2(width/2.0,height/2.0);
        for object in &self.pendulums {
            canvas.draw(
                &object.draw_line(&ctx, start_point).unwrap(),
                //graphics::DrawParam::default().scale(vec2(scale, scale)),
                graphics::DrawParam::default(),
            );
            canvas.draw(
                &object.draw_mass(&ctx, start_point).unwrap(),
                //graphics::DrawParam::default().scale(vec2(scale, scale)),
                graphics::DrawParam::default(),
            );
            start_point = object.end_point(start_point);
            // print!("{:?} ", object);
        }
        // println!();

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let mut c = conf::Conf::new();
    c.window_mode.resizable = true;
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    let state = State::new(&mut ctx)?;
    println!("Hello, world!");
    event::run(ctx, event_loop, state);
}
