use bevy::{color::palettes::css, prelude::*};
use bevy_potoo::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(GepPlugin);

    app.add_systems(Startup, setup);
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

impl GameState {
    pub fn update(
        mut gep: Gep,

        mut gs: Local<Self>,
        mut mesh: Local<Mesh2d>,
        mut mat: Local<MeshMaterial2d<ColorMaterial>>,

        mut commands: Commands,

        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        time: Res<Time>,
    ) {
        if gep.when_once(Ident::here()) {
            gs.boxes.push(Vec2::new(-100.0, 0.0));
            gs.boxes.push(Vec2::new(100.0, 0.0));
            gs.boxes.push(Vec2::new(0.0, 100.0));

            *mesh = Mesh2d(meshes.add(Mesh::from(Rectangle::new(50.0, 50.0))));
            *mat = MeshMaterial2d(materials.add(ColorMaterial::from(Color::from(css::ORANGE))));
        }

        for (i, b) in gs.boxes.iter().enumerate() {
            let ident = Ident::here_keyed(i);
            let e = gep.get(ident);

            if gep.when_init(ident) {
                println!("Initializing entity {:?}", e);
                commands.entity(e).insert((mesh.clone(), mat.clone()));
            }

            // if gep.when_activate(ident) {
            //     todo!();
            // }

            commands.entity(e).insert(Transform::from_xyz(
                b.x,
                b.y + (time.elapsed_secs().sin() * 50.0),
                0.0,
            ));

            // gep.on_deactivate(ident, |ec: &mut EntityCommands| todo!());
        }
    }
}
