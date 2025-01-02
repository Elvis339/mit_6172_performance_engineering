use rand::Rng;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn add(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn scale(&self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}

#[derive(Debug)]
struct Body {
    position: Vec2,
    velocity: Vec2,
    force: Vec2,
    mass: f64,
}

impl Default for Body {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            position: Vec2 {
                x: rng.gen_range(-10.0..10.0),
                y: rng.gen_range(-10.0..10.0),
            },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            force: Vec2 { x: 0.0, y: 0.0 },
            mass: rng.gen_range(1.0..100.0),
        }
    }
}

#[no_mangle]
fn update_position(bodies: &mut [Body], time_quantum: f64) {
    for body in bodies.iter_mut() {
        let new_velocity = body.force.scale(time_quantum / body.mass);
        body.position = body
            .position
            .add(body.velocity.add(new_velocity).scale(time_quantum / 2.0));
        body.velocity = new_velocity;
    }
}

fn simulate(bodies: &mut [Body], nsteps: i32, time_quantum: f64) {
    for _ in 0..nsteps {
        update_position(bodies, time_quantum);
    }
}

fn main() {
    let nbodies = 1000;
    let mut bodies: Vec<Body> = (0..nbodies).map(|_| Body::default()).collect();

    let time_quantum = 0.1;
    let nsteps = 10_000;

    simulate(&mut bodies, nsteps, time_quantum);

    for (i, body) in bodies.iter().enumerate() {
        println!(
            "Body {}: Position: ({:.2}, {:.2})",
            i, body.position.x, body.position.y
        );
    }
}
