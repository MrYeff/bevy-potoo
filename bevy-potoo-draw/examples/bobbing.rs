use bevy::{color::palettes::css, prelude::*};
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

#[derive(Default)]
struct GameState {
    boxes: Vec<Vec2>,
}

impl GameState {
    pub fn update(mut gs: Local<Self>, mut gep: Gep, mut draw: Draw, time: Res<Time>) {
        if gep.when_once(Ident::here()) {
            gs.boxes.push(Vec2::new(-100.0, 0.0));
            gs.boxes.push(Vec2::new(100.0, 0.0));
            gs.boxes.push(Vec2::new(0.0, 100.0));
        }

        for (i, b) in gs.boxes.iter().enumerate() {
            let ident = Ident::here_keyed(i);
            let bob = (time.elapsed_secs() * 2.0 + i as f32).sin() * 20.0;

            draw.draw_primitive(
                &mut gep,
                ident,
                Rectangle::new(50.0, 50.0),
                css::CRIMSON,
                Vec2::new(b.x, b.y + bob),
            );
        }
    }
}
