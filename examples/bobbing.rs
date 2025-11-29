use std::sync::Once;

use bevy::{color::palettes::css, mesh, prelude::*};
use bevy_potoo::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(GepPlugin);

    app.init_resource::<GameState>();
    app.add_systems(Startup, (setup, GameState::start));
    app.add_systems(Update, GameState::update.before(GepPlugin));

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
struct GameState {
    boxes: Vec<Vec2>,
}

static INIT: Once = Once::new();

impl GameState {
    pub fn start(mut gs: ResMut<Self>) {
        gs.boxes.push(Vec2::new(-100.0, 0.0));
        gs.boxes.push(Vec2::new(100.0, 0.0));
        gs.boxes.push(Vec2::new(0.0, 100.0));
    }

    pub fn update(
        gs: ResMut<Self>,
        mut gep: Gep,
        time: Res<Time>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut mesh: Local<Mesh2d>,
        mut mat: Local<MeshMaterial2d<ColorMaterial>>,
    ) {
        INIT.call_once(|| {
            *mesh = Mesh2d(meshes.add(Mesh::from(Rectangle::new(50.0, 50.0))));
            *mat = MeshMaterial2d(materials.add(ColorMaterial::from(Color::from(css::ORANGE))));
        });

        for (i, b) in gs.boxes.iter().enumerate() {
            let tf = Transform::from_xyz(b.x, b.y + (time.elapsed_secs().sin() * 50.0), 0.0);
            let mesh = mesh.clone();
            let mat = mat.clone();

            gep.target((Loc::here(), Key::from(i)))
                .once(move |ec: &mut EntityCommands| {
                    ec.insert((mesh, mat));
                })
                .on_update(move |ec: &mut EntityCommands| {
                    ec.insert(tf);
                });
            // .on_awake(f).on_sleep(f)
        }
    }
}
