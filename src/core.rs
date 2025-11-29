use crate::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::panic::Location;

type CallsiteId = u64;

#[derive(Component, Hash, Eq, PartialEq, Debug)]
pub struct ImmidiateId(CallsiteId, u64);

#[derive(Resource, Default)]
pub struct DrawNext {
    img_requests: HashMap<ImmidiateId, (Transform, Handle<Image>)>,
    prim_requests: HashMap<ImmidiateId, (Transform, Primitive)>,
}

impl Debug for DrawNext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DrawNext")
            .field("img_requests:", &self.img_requests.len())
            .field("prim_requests", &self.prim_requests.len())
            .finish()
    }
}

impl DrawNext {
    #[track_caller]
    pub fn draw_keyed(
        &mut self,
        iso: impl Into<Transform>,
        drawable: impl Into<Drawable>,
        key: impl Hash,
    ) {
        let callsite = (Location::caller() as *const Location) as u64;
        let key = hash_ident(&key);
        match drawable.into() {
            Drawable::Image(img) => {
                self.img_requests
                    .insert(ImmidiateId(callsite, key), (iso.into(), img));
            }
            Drawable::Primitive(prim) => {
                self.prim_requests
                    .insert(ImmidiateId(callsite, key), (iso.into(), prim));
            }
        };
    }

    #[track_caller]
    pub fn draw(&mut self, iso: impl Into<Transform>, drawable: impl Into<Drawable>) {
        let callsite = (Location::caller() as *const Location) as u64;
        match drawable.into() {
            Drawable::Image(img) => {
                self.img_requests
                    .insert(ImmidiateId(callsite, 0), (iso.into(), img));
            }
            Drawable::Primitive(prim) => {
                self.prim_requests
                    .insert(ImmidiateId(callsite, 0), (iso.into(), prim));
            }
        };
    }
}

fn hash_ident<T: Hash>(ident: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    ident.hash(&mut hasher);
    hasher.finish() + 1
}

pub(super) fn update_sprites(
    mut draw_next: ResMut<DrawNext>,
    mut entities: Query<(&ImmidiateId, &mut Transform, &mut Visibility, &mut Sprite)>,
    mut commands: Commands,
) {
    for (id, mut tf, mut vis, mut sprite) in entities.iter_mut() {
        if let Some((tf_new, img)) = draw_next.img_requests.remove(id) {
            *tf = tf_new;
            sprite.image = img;
            *vis = Visibility::Visible;
        } else {
            *vis = Visibility::Hidden;
        }
    }

    for (id, (tf_new, img)) in draw_next.img_requests.drain() {
        commands.spawn((id, tf_new, Visibility::Visible, Sprite::from_image(img)));
    }
}

struct PrebuildMeshes {
    circle_mesh: Handle<Mesh>,
    rect_mesh: Handle<Mesh>,
}

#[derive(Resource)]
pub(super) struct PrimMeshesMaterials {
    color_materials: HashMap<ColorKey, Handle<ColorMaterial>>,
    prebuilt_meshes: PrebuildMeshes,
}

pub(super) fn init_prim_meshes_materials(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let circle_mesh = meshes.add(Circle::default());
    let rect_mesh = meshes.add(Rectangle::default());

    commands.insert_resource(PrimMeshesMaterials {
        color_materials: HashMap::new(),
        prebuilt_meshes: PrebuildMeshes {
            circle_mesh,
            rect_mesh,
        },
    });
}

pub(super) fn update_primitives(
    mut draw_next: ResMut<DrawNext>,
    mut entities: Query<(
        &ImmidiateId,
        &mut Transform,
        &mut Visibility,
        &mut Mesh2d,
        &mut MeshMaterial2d<ColorMaterial>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut prim_meshes_materials: ResMut<PrimMeshesMaterials>,
    mut commands: Commands,
) {
    for (id, mut tf, mut vis, mut mesh2d, mut mat2d) in entities.iter_mut() {
        if let Some((tf_new, prim)) = draw_next.prim_requests.remove(id) {
            let mesh_handle = match prim.shape {
                PrimitiveShape::Circle => prim_meshes_materials.prebuilt_meshes.circle_mesh.clone(),
                PrimitiveShape::Rect => prim_meshes_materials.prebuilt_meshes.rect_mesh.clone(),
            };

            *tf = tf_new;
            mesh2d.0 = mesh_handle;

            let color_material = prim_meshes_materials
                .color_materials
                .entry(prim.color.into())
                .or_insert_with(|| materials.add(ColorMaterial::from(prim.color)))
                .clone();

            mat2d.0 = color_material;

            *vis = Visibility::Visible;
        } else {
            *vis = Visibility::Hidden;
        }
    }

    for (id, (tf_new, prim)) in draw_next.prim_requests.drain() {
        let mesh_handle = match prim.shape {
            PrimitiveShape::Circle => prim_meshes_materials.prebuilt_meshes.circle_mesh.clone(),
            PrimitiveShape::Rect => prim_meshes_materials.prebuilt_meshes.rect_mesh.clone(),
        };

        let color_material = prim_meshes_materials
            .color_materials
            .entry(prim.color.into())
            .or_insert_with(|| materials.add(ColorMaterial::from(prim.color)))
            .clone();

        commands.spawn((
            id,
            tf_new,
            Visibility::Visible,
            Mesh2d(mesh_handle),
            MeshMaterial2d(color_material),
        ));
    }
}

#[derive(Hash, Eq, PartialEq)]
struct ColorKey([u8; 4]);

impl From<Color> for ColorKey {
    fn from(color: Color) -> Self {
        let rgba = color.to_srgba();
        ColorKey([
            (rgba.red * 255.0) as u8,
            (rgba.green * 255.0) as u8,
            (rgba.blue * 255.0) as u8,
            (rgba.alpha * 255.0) as u8,
        ])
    }
}
