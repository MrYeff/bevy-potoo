use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    panic::Location,
};

#[derive(Component, Hash, Eq, PartialEq, Debug)]
enum ImmidiateId {
    Single { callsite: u64 },
    Keyed { callsite: u64, ident: u64 },
    GlobalKeyed(Box<str>),
}

#[derive(Resource, Default)]
pub struct FlurpStorage(
    HashMap<ImmidiateId, Box<dyn FnOnce(&mut EntityCommands) + 'static + Send + Sync>>,
);

impl FlurpStorage {
    #[track_caller]
    pub fn flurp(&mut self, f: impl FnOnce(&mut EntityCommands) + 'static + Send + Sync) {
        let loc: &'static Location = Location::caller();
        let callsite = (loc as *const Location) as u64;

        assert!(
            self.0
                .insert(ImmidiateId::Single { callsite }, Box::new(f))
                .is_none(),
        );
    }

    #[track_caller]
    pub fn flurp_keyed<K: IntoIdent + Debug>(
        &mut self,
        key: K,
        f: impl FnOnce(&mut EntityCommands) + 'static + Send + Sync,
    ) {
        let loc: &'static Location = Location::caller();
        let callsite = (loc as *const Location) as u64;
        let ident = key.into_ident();

        assert!(
            self.0
                .insert(ImmidiateId::Keyed { callsite, ident }, Box::new(f))
                .is_none(),
        );
    }

    #[track_caller]
    pub fn flurp_insert(&mut self, b: impl Bundle) {
        let loc: &'static Location = Location::caller();
        let callsite = (loc as *const Location) as u64;

        assert!(
            self.0
                .insert(
                    ImmidiateId::Single { callsite },
                    Box::new(move |ec| {
                        ec.insert(b);
                    })
                )
                .is_none(),
        );
    }

    #[track_caller]
    pub fn flurp_insert_keyed<K: IntoIdent + Debug>(&mut self, key: K, b: impl Bundle) {
        let loc: &'static Location = Location::caller();
        let callsite = (loc as *const Location) as u64;
        let ident = key.into_ident();

        assert!(
            self.0
                .insert(
                    ImmidiateId::Keyed { callsite, ident },
                    Box::new(move |ec| {
                        ec.insert(b);
                    })
                )
                .is_none(),
        );
    }

    // pub fn flurp_this<'a>(
    //     &mut self,
    //     id: impl Into<Cow<'a, str>>,
    //     f: impl Fn(&mut EntityCommands) + 'static + Send + Sync,
    // ) {
    //     let id = id.into();
    //     self.0.insert(
    //         ImmidiateId::GlobalKeyed(id.into_owned().into_boxed_str()),
    //         Box::new(f),
    //     );
    // }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Active;

impl FlurpStorage {
    fn apply(
        mut diffs: ResMut<Self>,
        enabled: Query<(Entity, &ImmidiateId), With<Active>>,
        disabled: Query<(Entity, &ImmidiateId), Without<Active>>,
        mut commands: Commands,
    ) {
        for (e, id) in disabled {
            if let Some(t) = diffs.0.remove(id) {
                t(&mut commands.entity(e).insert(Active));
            }
        }

        for (e, id) in enabled {
            if let Some(t) = diffs.0.remove(id) {
                t(&mut commands.entity(e));
            } else {
                commands.entity(e).remove::<Active>();
            }
        }

        for (id, t) in diffs.0.drain() {
            t(&mut commands.spawn((id, Active)));
        }
    }
}

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub struct FlurpPlugin;

impl Plugin for FlurpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlurpStorage>();
        app.add_systems(Update, (FlurpStorage::apply).in_set(FlurpPlugin));
    }
}

pub trait IntoIdent {
    fn into_ident(self) -> u64;
}

impl IntoIdent for u64 {
    fn into_ident(self) -> u64 {
        self
    }
}

impl IntoIdent for usize {
    fn into_ident(self) -> u64 {
        self as u64
    }
}
