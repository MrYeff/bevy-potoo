use bevy::{color::palettes::css, prelude::*};
use bevy_potoo::prelude::*;
use rand::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(PotooPlugin);

    app.init_resource::<GameState>();
    app.init_resource::<ParticleSystem>();

    app.add_systems(Startup, (setup, GameState::start));
    app.add_systems(Update, (GameState::update_player, particle_update));

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
struct GameState {
    player_pos: Vec2,
}

impl GameState {
    pub fn start(mut gs: ResMut<Self>) {
        gs.player_pos = Vec2::ZERO;
    }

    /// Simple WASD movement like in your first example
    pub fn update_player(mut gs: ResMut<Self>, time: Res<Time>, input: Res<ButtonInput<KeyCode>>) {
        let mut dir = Vec2::ZERO;

        if input.pressed(KeyCode::KeyA) {
            dir.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            dir.x += 1.0;
        }
        if input.pressed(KeyCode::KeyW) {
            dir.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            dir.y -= 1.0;
        }

        if dir.length_squared() > 0.0 {
            dir = dir.normalize();
        }

        let speed = 300.0;
        gs.player_pos += dir * speed * time.delta_secs();
    }
}

// -------------------
// Particle system data
// -------------------

#[derive(Resource, Default)]
struct ParticleSystem {
    particles: Vec<Particle>,
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    lifetime: f32,
    age: f32,
    start_color: Color,
    end_color: Color,
    start_size: f32,
    end_size: f32,
}

// -------------------
// Particle update + draw
// -------------------

fn particle_update(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    gs: Res<GameState>,
    mut particles: ResMut<ParticleSystem>,
    mut d: ResMut<DrawNext>,
) {
    let dt = time.delta_secs();

    // Spawn a small burst every frame while Space is held
    if input.pressed(KeyCode::Space) {
        let mut rng = rand::rng();
        spawn_burst(&mut particles, gs.player_pos, 20, &mut rng);
    }

    // Update + draw all particles
    for (id, p) in particles.particles.iter_mut().enumerate() {
        p.age += dt;
        if p.age >= p.lifetime {
            continue;
        }

        // Integrate motion
        p.pos += p.vel * dt;

        let t = (p.age / p.lifetime).clamp(0.0, 1.0);
        let size = lerp(p.start_size, p.end_size, t);
        let color = lerp_color(p.start_color, p.end_color, t);

        d.draw_keyed(
            Transform::from_translation(p.pos.extend(0.0)).with_scale(Vec3::splat(size)),
            Drawable::from(Primitive::circle(color)),
            id,
        );
    }

    // Remove dead particles
    particles.particles.retain(|p| p.age < p.lifetime);
}

// -------------------
// Spawning helpers
// -------------------

fn spawn_burst(system: &mut ParticleSystem, origin: Vec2, count: usize, rng: &mut impl Rng) {
    for _ in 0..count {
        // random direction + speed
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let speed = rng.random_range(80.0..220.0);
        let dir = Vec2::new(angle.cos(), angle.sin());
        let vel = dir * speed;

        let lifetime = rng.random_range(0.4..1.0);

        let start_size = rng.random_range(8.0..16.0);
        let end_size = 0.0;

        let start_color: Color = css::ORANGE_RED.into();
        let end_color: Color = css::YELLOW.into();

        system.particles.push(Particle {
            pos: origin,
            vel,
            lifetime,
            age: 0.0,
            start_color,
            end_color,
            start_size,
            end_size,
        });
    }
}

// -------------------
// Small lerp helpers
// -------------------

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Lerp between two Colors in sRGBA space
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let a = a.to_srgba();
    let b = b.to_srgba();

    Color::srgba(
        lerp(a.red, b.red, t),
        lerp(a.green, b.green, t),
        lerp(a.blue, b.blue, t),
        lerp(a.alpha, b.alpha, t),
    )
}
