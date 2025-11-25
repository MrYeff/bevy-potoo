use bevy::{color::palettes::css, prelude::*};
use bevy_potoo::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(PotooPlugin);

    app.init_resource::<GameState>();
    app.add_systems(Startup, (setup, GameState::start));
    app.add_systems(Update, GameState::update);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
struct GameState {
    boxes: Vec<Vec2>,
}

impl GameState {
    pub fn start(mut gs: ResMut<Self>) {
        gs.boxes.push(Vec2::new(-100.0, 0.0));
        gs.boxes.push(Vec2::new(100.0, 0.0));
        gs.boxes.push(Vec2::new(0.0, 100.0));
    }

    pub fn update(mut gs: ResMut<Self>, mut d: ResMut<DrawNext>, time: Res<Time>) {
        for b in gs.boxes.iter() {
            d.draw_keyed(
                Transform::from_translation(Vec3::new(
                    b.x,
                    b.y + (time.elapsed_secs().sin() * 50.0),
                    0.0,
                ))
                .with_scale(Vec3::splat(32.0)),
                Drawable::Primitive(Primitive {
                    color: css::RED.into(),
                    shape: PrimitiveShape::Rect,
                }),
                b.as_ivec2(),
            );
        }
    }
}
