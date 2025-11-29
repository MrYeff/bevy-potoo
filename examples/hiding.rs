use bevy::{color::palettes::css, prelude::*};
use bevy_potoo::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(PotooPlugin);

    app.init_resource::<GameState>();
    app.add_systems(Startup, setup);
    app.add_systems(Update, GameState::update);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
struct GameState {}

impl GameState {
    pub fn update(mut d: ResMut<DrawNext>, time: Res<Time>) {
        let t = time.elapsed_secs().floor() as i32;
        if t % 2 == 0 {
            d.draw(
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(64.0)),
                Drawable::from(Primitive::circle(css::BLUE.into())),
            );
        }
    }
}
