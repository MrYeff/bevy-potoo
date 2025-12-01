use bevy::{color::palettes::css, prelude::*};
use bevy_potoo::prelude::*;
use bevy_potoo_draw::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins((GepPlugin, DrawPlugin));

    app.add_systems(Startup, setup);
    app.add_systems(Update, update.before(GepPlugin));

    app.run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn update(mut gep: Gep, mut draw: Draw, time: Res<Time>) {
    let t = time.elapsed_secs().floor() as i32;
    if t % 2 == 0 {
        draw.draw_primitive(
            &mut gep,
            Ident::here(),
            Circle::new(100.0),
            css::BLUE,
            Vec2::new(0.0, 0.0),
        );
    }
}
