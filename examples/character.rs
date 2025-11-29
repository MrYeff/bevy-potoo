use bevy::{asset, color::palettes::css, prelude::*};
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
    player_pos: Vec2,
}

impl GameState {
    pub fn start(mut gs: ResMut<Self>) {
        gs.player_pos = Vec2::ZERO;
    }

    pub fn update(
        mut gs: ResMut<Self>,
        mut d: ResMut<DrawNext>,
        time: Res<Time>,
        input: Res<ButtonInput<KeyCode>>,
        assets: Res<AssetServer>,
    ) {
        if input.pressed(KeyCode::KeyA) {
            gs.player_pos.x -= 300.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyD) {
            gs.player_pos.x += 300.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyW) {
            gs.player_pos.y += 300.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyS) {
            gs.player_pos.y -= 300.0 * time.delta_secs();
        }

        d.draw(
            Transform::from_translation(Vec3::new(gs.player_pos.x, gs.player_pos.y, 0.0))
                .with_scale(Vec3::splat(1.0)),
            Drawable::from(assets.load("character.png")),
        );

        d.draw(
            Transform::from_translation(Vec3::new(gs.player_pos.x + 100.0, gs.player_pos.y, 0.0))
                .with_scale(Vec3::splat(1.0)),
            Drawable::from(assets.load("character.png")),
        );
    }
}
