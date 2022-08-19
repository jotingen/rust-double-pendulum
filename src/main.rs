use std::f32::consts::PI;

use ggez::*;
use glam::*;
use rand::Rng;

const G: f32 = 6.674e-11;
const g: f32 = 9.81;

#[derive(Clone, Copy, Debug)]
struct Pendulum {
    length: f32,
    angle: f32,
    angle_velocity: f32,
    angle_acceleration: f32,
    mass: f32,
    color: graphics::Color,
}

impl Pendulum {
    fn new(ctx: &Context) -> Pendulum {
        let mut rng = rand::thread_rng();
        let length: f32 = rng.gen_range(50.0..200.0);
        let angle: f32 = rng.gen_range(0.0..2.0 * PI);
        let mass: f32 = rng.gen_range(10.0..50.0);
        Pendulum {
            length,
            angle,
            angle_acceleration: 0.0,
            angle_velocity: 0.0,
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
    fn with_mass(mut self, mass: f32) -> Pendulum {
        self.mass = mass;
        self
    }
    fn with_color(mut self, color: graphics::Color) -> Pendulum {
        self.color = color;
        self
    }
    fn calculate_force(&mut self, num: usize, other: &Pendulum) {
        if num == 0 {
            let mass_1 = self.mass;
            let length_1 = self.length;
            let angle_1 = self.angle;
            let angle_velocity_1 = self.angle_velocity;
            let mass_2 = other.mass;
            let length_2 = other.length;
            let angle_2 = other.angle;
            let angle_velocity_2 = other.angle_velocity;
            self.angle_acceleration = (-g * (2.0 * mass_1 + mass_2) * angle_1.sin()
                - mass_2 * g * (angle_1 - 2.0 * angle_2).sin()
                - 2.0
                    * (angle_1 - angle_2).sin()
                    * mass_2
                    * (angle_velocity_2 * angle_velocity_2 * length_2
                        + angle_velocity_1
                            * angle_velocity_1
                            * length_1
                            * (angle_1 - angle_2).cos()))
                / (length_1
                    * (2.0 * mass_1 + mass_2 - mass_2 * (2.0 * angle_1 - 2.0 * angle_2).cos()));
        } else if num == 1 {
            let mass_1 = other.mass;
            let length_1 = other.length;
            let angle_1 = other.angle;
            let angle_velocity_1 = other.angle_velocity;
            let mass_2 = self.mass;
            let length_2 = self.length;
            let angle_2 = self.angle;
            let angle_velocity_2 = self.angle_velocity;
            self.angle_acceleration = (2.0
                * (angle_1 - angle_2).sin()
                * (angle_velocity_1 * angle_velocity_1 * length_1 * (mass_1 + mass_2)
                    + g * (mass_1 + mass_2) * angle_1.cos()
                    + angle_velocity_2
                        * angle_velocity_2
                        * length_2
                        * mass_2
                        * (angle_1 - angle_2).cos()))
                / (length_1
                    * (2.0 * mass_1 + mass_2 - mass_2 * (2.0 * angle_1 - 2.0 * angle_2).cos()));
        }
    }
    fn update(&mut self, dt: f32) {
        self.angle_velocity += self.angle_acceleration * dt;
        self.angle += self.angle_velocity * dt;
    }
    fn end_point(&self, start_point: Vec2) -> Vec2 {
        start_point
            + vec2(
                self.length * self.angle.sin(),
                self.length * self.angle.cos(),
            )
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
    points: Vec<Vec2>,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let mut rng = rand::thread_rng();
        let mut pendulums: Vec<Pendulum> = vec![];
        let mut points: Vec<Vec2> = vec![];
        for _ in 0..2 {
            pendulums.push(Pendulum::new(&ctx));
        }
        Ok(State {
            dt: ctx.time.delta(),
            pendulums,
            points,
        })
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();

        let mut other = self.pendulums[1].clone();
        self.pendulums[0].calculate_force(0, &other);
        other = self.pendulums[0].clone();
        self.pendulums[1].calculate_force(1, &other);

        self.pendulums[0].update(1.0 / 15.0);
        self.pendulums[1].update(1.0 / 15.0);

        self.points
            .push(self.pendulums[1].end_point(self.pendulums[0].end_point(vec2(0.0, 0.0))));

        if self.points.len() > 200 {
            self.points.remove(0);
        }

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

        let mut start_point: Vec2 = vec2(width / 2.0, height / 2.0);
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

        for point in &self.points {
            canvas.draw(
                &graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    (vec2(width / 2.0, height / 2.0)+*point),
                    1.0,
                    2.0,
                    graphics::Color::WHITE,
                )
                .unwrap(),
                graphics::DrawParam::default(),
            );
        }
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
