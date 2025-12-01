use bevy::prelude::*;
use bevy_potoo::prelude::*;
use bevy_potoo_draw::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins((GepPlugin, DrawPlugin));

    app.add_systems(Startup, setup);
    app.add_systems(Update, GameState::update.before(GepPlugin));

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
    pub fn update(
        mut gs: Local<Self>,

        mut gep: Gep,
        mut draw: Draw,

        time: Res<Time>,
        input: Res<ButtonInput<KeyCode>>,
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

        let ident = Ident::here();
        draw.draw_sprite(
            &mut gep,
            ident,
            "character.png",
            gs.player_pos,
            Vec2::splat(1.0),
        );
    }
}
