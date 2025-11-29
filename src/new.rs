use bevy::{
    ecs::system::SystemParam, platform::collections::HashMap, prelude::*,
    sprite_render::Material2d, state::commands,
};
use std::{
    cell::RefMut,
    hash::{DefaultHasher, Hash, Hasher},
    panic::Location,
};

#[derive(Component, Hash, Eq, PartialEq, Debug)]
pub struct ImmidiateId {
    callsite: u64,
    ident: u64,
}

impl ImmidiateId {
    fn new_here() -> Self {
        let callsite = (Location::caller() as *const Location) as u64;
        Self { callsite, ident: 0 }
    }

    fn new_here_with_key(ident: u64) -> Self {
        let callsite = (Location::caller() as *const Location) as u64;
        Self { callsite, ident }
    }
}

#[derive(Resource)]
struct Diffs<T: ApplyDiff>(HashMap<ImmidiateId, T>);

impl<T: ApplyDiff> Diffs<T> {
    fn into_constructor<'a, C>(&'a mut self, cfg: C) -> DiffsConstructor<'a, T, C> {
        DiffsConstructor::new(self, cfg)
    }

    fn into_default_constructor<'a, C: Default>(&'a mut self) -> DiffsConstructor<'a, T, C> {
        DiffsConstructor::new(self, default())
    }
}

struct DiffsConstructor<'a, T: ApplyDiff, C> {
    cfg: C,
    diffs: &'a mut Diffs<T>,
}

impl<'a, T: ApplyDiff, C> DiffsConstructor<'a, T, C> {
    pub fn new(diffs: &'a mut Diffs<T>, cfg: C) -> Self {
        Self { cfg, diffs: diffs }
    }

    fn set_cfg(&mut self, c: C) {
        self.cfg = c;
    }

    #[track_caller]
    fn execute<'b, TS>(&'b mut self, ts: TS)
    where
        T: From<(TS, &'b C)>,
    {
        self.execute_redirect(ts);
    }

    /// use track_caller on caller
    fn execute_redirect<'b, TS>(&'b mut self, ts: TS)
    where
        T: From<(TS, &'b C)>,
    {
        let t = T::from((ts, &self.cfg));
        let id = ImmidiateId::new_here();
        assert!(self.diffs.0.insert(id, t).is_none());
    }

    #[track_caller]
    fn execute_keyed<'b, TS>(&'b mut self, ts: TS, key: impl Hash)
    where
        T: From<(TS, &'b C)>,
    {
        self.execute_keyed_redirect(ts, key);
    }

    /// use track_caller on caller
    fn execute_keyed_redirect<'b, TS>(&'b mut self, ts: TS, key: impl Hash)
    where
        T: From<(TS, &'b C)>,
    {
        let t = T::from((ts, &self.cfg));
        let id = ImmidiateId::new_here_with_key(hash_ident(&key));
        assert!(self.diffs.0.insert(id, t).is_none());
    }
}

fn hash_ident<T: Hash>(ident: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    ident.hash(&mut hasher);
    hasher.finish() + 1
}

trait ApplyDiff {
    type FamilyMarker: Component;
    type Params<'w, 's>: SystemParam;

    type TInit;
    type TActivate;
    type TUpdate;

    fn split(self) -> (Self::TInit, Self::TActivate, Self::TUpdate);

    /// from unknown -> enabled (init + update)
    fn init(t: Self::TInit, params: &mut Self::Params<'_, '_>, ec: &mut EntityCommands);
    /// from enabled -> enabled
    fn update(t: Self::TUpdate, params: &mut Self::Params<'_, '_>, ec: &mut EntityCommands);
    /// from disabled -> enabled (activate + update)
    fn activate(t: Self::TActivate, params: &mut Self::Params<'_, '_>, ec: &mut EntityCommands);
    /// from enabled -> disabled
    fn deactivate(params: &mut Self::Params<'_, '_>, ec: &mut EntityCommands);
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Active;

impl<T: ApplyDiff + Send + Sync + 'static> Diffs<T> {
    fn apply_diffs(
        mut diffs: ResMut<Self>,
        enabled: Query<(Entity, &ImmidiateId), (With<T::FamilyMarker>, With<Active>)>,
        disabled: Query<(Entity, &ImmidiateId), (With<T::FamilyMarker>, Without<Active>)>,
        mut commands: Commands,
        mut params: T::Params<'_, '_>,
    ) {
        for (e, id) in disabled {
            if let Some(t) = diffs.0.remove(id) {
                let mut ec = commands.entity(e);
                ec.insert(Active);
                let (_, ta, tu) = t.split();
                T::activate(ta, &mut params, &mut ec);
                T::update(tu, &mut params, &mut ec);
            }
        }

        for (e, id) in enabled {
            if let Some(t) = diffs.0.remove(id) {
                let (_, _, tu) = t.split();
                T::update(tu, &mut params, commands.entity(e).insert(Active));
            } else {
                T::deactivate(&mut params, commands.entity(e).remove::<Active>());
            }
        }

        for (id, t) in diffs.0.drain() {
            let mut ec = commands.spawn((id, Active));
            let (ti, _, tu) = t.split();
            T::init(ti, &mut params, &mut ec);
            T::update(tu, &mut params, &mut ec);
        }
    }
}

// move somewhere

#[derive(Resource, Default)]
struct ColorMaterials(HashMap<ColorKey, Handle<ColorMaterial>>);

impl ColorMaterials {
    fn get_or_insert(
        &mut self,
        color: Color,
        materials: &mut Assets<ColorMaterial>,
    ) -> Handle<ColorMaterial> {
        self.0
            .entry(color.into())
            .or_insert_with(|| materials.add(ColorMaterial::from(color)))
            .clone()
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

// move to draw rect

struct DrawRect(Transform, Handle<ColorMaterial>, Rectangle);

#[derive(Component)]
struct DrawRectFamilyMarker;

/// Mesh of a 1x1 quad
#[derive(Resource)]
struct RectMeshRef(Handle<Mesh>);

#[derive(SystemParam)]
struct DrawRectParams<'w> {
    mesh: Res<'w, RectMeshRef>,
}

impl ApplyDiff for DrawRect {
    type FamilyMarker = DrawRectFamilyMarker;

    type Params<'w, 's> = DrawRectParams<'w>;

    type TInit = ();
    type TActivate = ();
    type TUpdate = Self;

    fn split(self) -> (Self::TInit, Self::TActivate, Self::TUpdate) {
        ((), (), self)
    }

    fn init(_: (), params: &mut Self::Params<'_, '_>, ec: &mut EntityCommands) {
        ec.insert((Mesh2d(params.mesh.0.clone()), Visibility::Visible));
    }

    fn update(t: Self::TUpdate, _params: &mut Self::Params<'_, '_>, ec: &mut EntityCommands) {
        let DrawRect(tf, mat, rect) = t;
        let tf = tf.with_scale(tf.scale * rect.size().extend(1.0));

        ec.insert((tf, MeshMaterial2d(mat)));
    }

    fn activate(_: Self::TActivate, _params: &mut Self::Params<'_, '_>, ec: &mut EntityCommands) {
        ec.insert(Visibility::Visible);
    }

    fn deactivate(_: &mut Self::Params<'_, '_>, ec: &mut EntityCommands) {
        ec.insert(Visibility::Hidden);
    }
}

type DrawRectDiffs = Diffs<DrawRect>;

struct DrawRectConfig {
    pub color: Handle<ColorMaterial>,
}

impl From<((Transform, Rectangle), &DrawRectConfig)> for DrawRect {
    fn from(((tf, rect), cfg): ((Transform, Rectangle), &DrawRectConfig)) -> Self {
        DrawRect(tf, cfg.color.clone(), rect)
    }
}

type DrawRectConstructor<'a> = DiffsConstructor<'a, DrawRect, DrawRectConfig>;

trait DrawRectConstructorExt<'a> {
    fn set_material(&'a mut self, material: Handle<ColorMaterial>);
    fn single(&'a mut self, tf: impl Into<Transform>, rect: impl Into<Rectangle>);
    fn keyed(&'a mut self, tf: impl Into<Transform>, rect: impl Into<Rectangle>, key: impl Hash);
}

impl<'a> DrawRectConstructorExt<'a> for DrawRectConstructor<'a> {
    fn set_material(&'a mut self, material: Handle<ColorMaterial>) {
        self.set_cfg(DrawRectConfig { color: material });
    }

    #[track_caller]
    fn single(&'a mut self, tf: impl Into<Transform>, rect: impl Into<Rectangle>) {
        self.execute_redirect((tf.into(), rect.into()));
    }

    #[track_caller]
    fn keyed(&'a mut self, tf: impl Into<Transform>, rect: impl Into<Rectangle>, key: impl Hash) {
        self.execute_keyed_redirect((tf.into(), rect.into()), key);
    }
}

/* API:
let draw = DrawSystemParam -> DrawRect
let draw = draw.into_ctor();

draw.rect.set_material(mat);
draw.rect.single(Transform:from_xyz(0.0,0.0,0.0), Rectangle::new(Vec2::ZERO, Vec2::splat(100.0)));
*/
