use std::sync::Once;

use bevy::{color::palettes::css, mesh, prelude::*};
use bevy_potoo::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(FlurpPlugin);

    app.init_resource::<GameState>();
    app.add_systems(Startup, (setup, GameState::start));
    app.add_systems(Update, GameState::update.before(FlurpPlugin));

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
        mut d: ResMut<FlurpStorage>,
        time: Res<Time>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut mesh_mat: Local<(Mesh2d, MeshMaterial2d<ColorMaterial>)>,
    ) {
        INIT.call_once(|| {
            mesh_mat.0 = Mesh2d(meshes.add(Mesh::from(Rectangle::new(50.0, 50.0))));
            mesh_mat.1 =
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::from(css::ORANGE))));
        });

        for (i, b) in gs.boxes.iter().enumerate() {
            d.flurp_insert_keyed(
                i,
                (
                    Transform::from_xyz(b.x, b.y + (time.elapsed_secs().sin() * 50.0), 0.0),
                    mesh_mat.0.clone(),
                    mesh_mat.1.clone(),
                ),
            );
        }
    }
}
