use bevy::ecs::system::SystemParam;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_potoo::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Ctx>();
}

#[derive(SystemParam)]
pub struct DrawPrimitive<'w, 's> {
    ctx: ResMut<'w, Ctx>,
    commands: Commands<'w, 's>,
    materials: ResMut<'w, Assets<ColorMaterial>>,
}

#[derive(Resource)]
struct Ctx {
    mesh_unit_circle: Mesh2d,
    mesh_unit_rect: Mesh2d,
    mats: HashMap<ColorKey, Handle<ColorMaterial>>,
}

impl DrawPrimitive<'_, '_> {
    pub fn draw(
        &mut self,
        gep: &mut Gep,
        ident: Ident,
        primitive: impl Into<Primitive>,
        color: impl Into<Color>,
        iso: impl Into<Isometry2d>,
    ) {
        let primitive = primitive.into();
        let color = color.into();
        let iso = iso.into();

        let e = gep.get(ident);

        if gep.when_once(ident) {
            let mat = self
                .ctx
                .mats
                .entry(color.into())
                .or_insert_with(|| self.materials.add(ColorMaterial::from(color)))
                .clone();

            let mesh = match primitive {
                Primitive::Circle { .. } => self.ctx.mesh_unit_circle.clone(),
                Primitive::Rectangle { .. } => self.ctx.mesh_unit_rect.clone(),
            };

            self.commands
                .entity(e)
                .insert((mesh.clone(), MeshMaterial2d(mat)));
        }

        if gep.when_activate(ident) {
            self.commands.entity(e).insert(Visibility::Visible);
        }

        let scale = match primitive {
            Primitive::Circle(c) => Vec3::splat(c.radius),
            Primitive::Rectangle(r) => r.size().extend(1.0),
        };

        self.commands.entity(e).insert(Transform {
            translation: Vec3::new(iso.translation.x, iso.translation.y, 0.0),
            rotation: Quat::from_rotation_z(iso.rotation.as_radians()),
            scale,
        });

        gep.on_deactivate(ident, |ec: &mut EntityCommands| {
            ec.insert(Visibility::Hidden);
        });
    }
}

impl FromWorld for Ctx {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();

        Self {
            mesh_unit_circle: Mesh2d(meshes.add(Mesh::from(Circle::new(1.0)))),
            mesh_unit_rect: Mesh2d(meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)))),
            mats: HashMap::new(),
        }
    }
}

pub enum Primitive {
    Circle(Circle),
    Rectangle(Rectangle),
}

impl From<Circle> for Primitive {
    fn from(p: Circle) -> Self {
        Primitive::Circle(p)
    }
}

impl From<Rectangle> for Primitive {
    fn from(p: Rectangle) -> Self {
        Primitive::Rectangle(p)
    }
}

#[derive(Hash, PartialEq, Eq)]
struct ColorKey([u8; 4]);

impl From<Color> for ColorKey {
    fn from(c: Color) -> Self {
        let srgb = c.to_srgba();
        ColorKey([
            (srgb.red * 255.0) as u8,
            (srgb.green * 255.0) as u8,
            (srgb.blue * 255.0) as u8,
            (srgb.alpha * 255.0) as u8,
        ])
    }
}
