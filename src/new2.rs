use bevy::{platform::collections::HashMap, prelude::*};
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
    HashMap<ImmidiateId, Box<dyn Fn(&mut EntityCommands) + 'static + Send + Sync>>,
);

impl FlurpStorage {
    #[track_caller]
    pub fn flurp(&mut self, f: impl Fn(&mut EntityCommands) + 'static + Send + Sync) {
        let loc: &'static Location = Location::caller();
        let callsite = (loc as *const Location) as u64;

        assert!(
            self.0
                .insert(ImmidiateId::Single { callsite }, Box::new(f))
                .is_none(),
            "Duplicate flurp at callsite {:?}",
            loc
        );
    }

    #[track_caller]
    pub fn flurp_keyed<K: Hash + Debug>(
        &mut self,
        key: K,
        f: impl Fn(&mut EntityCommands) + 'static + Send + Sync,
    ) {
        let loc: &'static Location = Location::caller();
        let callsite = (loc as *const Location) as u64;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        let ident = hasher.finish();

        assert!(
            self.0
                .insert(ImmidiateId::Keyed { callsite, ident }, Box::new(f))
                .is_none(),
            "Duplicate flurp_keyed at callsite {:?} with key {:?}",
            loc,
            key
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
                let mut ec = commands.entity(e);
                ec.insert(Active);
                t(&mut ec);
            }

            for (e, id) in enabled {
                if let Some(t) = diffs.0.remove(id) {
                    t(commands.entity(e).insert(Active));
                } else {
                    commands.entity(e).remove::<Active>();
                }
            }

            for (id, t) in diffs.0.drain() {
                let mut ec = commands.spawn((id, Active));
                t(&mut ec);
            }
        }
    }
}

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub struct PotooPlugin;

impl Plugin for PotooPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlurpStorage>();
        app.add_systems(Update, (FlurpStorage::apply).in_set(PotooPlugin));
    }
}
