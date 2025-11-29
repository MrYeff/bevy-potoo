use crate::fun::EntityFn;
use crate::ident::{Ident, Key, Loc};
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::{ecs::system::SystemParam, platform::collections::HashMap};
use std::fmt::Debug;
use std::mem;
use std::panic::Location;

enum Status {
    Active,
    Inactive,
}

#[derive(Resource, Default)]
struct GepData {
    ident_to_entity: HashMap<Ident, Entity>,

    all_calls: HashSet<(Loc, Option<Key>)>,
    all_calls_next: Vec<(Loc, Option<Key>)>,

    prev_calls: HashSet<(Loc, Option<Key>)>,
    prev_calls_next: Vec<(Loc, Option<Key>)>,

    fn_deactivate: HashMap<(Loc, Option<Key>), Vec<(Entity, EntityFn)>>,
}

/// Global Entity Protocol
#[derive(SystemParam)]
pub struct Gep<'w, 's> {
    data: ResMut<'w, GepData>,
    commands: Commands<'w, 's>,
}

pub struct GepWithTarget<'a, 'w, 's> {
    gep: &'a mut Gep<'w, 's>,
    ident: Ident,
    cid: (Loc, Option<Key>),
}

impl<'w, 's> Gep<'w, 's> {
    #[track_caller]
    pub fn target<'a>(&'a mut self, ident: impl Into<Ident>) -> GepWithTarget<'a, 'w, 's> {
        let ident = ident.into();
        let cid = match ident.clone() {
            Ident::Loc(loc) => (loc, None),
            Ident::Key(key) => (Loc::from(Location::caller()), Some(key)),
            Ident::LocAndKey(loc, key) => (loc, Some(key)),
        };

        self.data.all_calls_next.push(cid.clone());
        self.data.prev_calls_next.push(cid.clone());

        GepWithTarget {
            gep: self,
            ident: ident,
            cid: cid,
        }
    }
}

impl<'a, 'w, 's> GepWithTarget<'a, 'w, 's> {
    pub fn once(self, f: impl Into<EntityFn>) -> Self {
        let entity = *self
            .gep
            .data
            .ident_to_entity
            .entry(self.ident.clone())
            .or_insert_with_key(|ident| self.gep.commands.spawn(ident.clone()).id());

        if !self.gep.data.all_calls.contains(&self.cid) {
            f.into().0(&mut self.gep.commands.entity(entity));
        }

        return self;
    }

    pub fn on_awake(self, f: impl Into<EntityFn>) -> Self {
        let entity = *self
            .gep
            .data
            .ident_to_entity
            .entry(self.ident.clone())
            .or_insert_with_key(|ident| self.gep.commands.spawn(ident.clone()).id());

        if !self.gep.data.prev_calls.contains(&self.cid) {
            f.into().0(&mut self.gep.commands.entity(entity));
        }

        return self;
    }

    pub fn on_update(self, f: impl Into<EntityFn>) -> Self {
        let entity = *self
            .gep
            .data
            .ident_to_entity
            .entry(self.ident.clone())
            .or_insert_with_key(|ident| self.gep.commands.spawn(ident.clone()).id());

        f.into().0(&mut self.gep.commands.entity(entity));

        return self;
    }

    pub fn on_sleep(self, f: impl Into<EntityFn>) -> Self {
        let Some(entity) = self.gep.data.ident_to_entity.get(&self.ident).cloned() else {
            return self;
        };

        self.gep
            .data
            .fn_deactivate
            .entry(self.cid.clone())
            .or_default()
            .push((entity, f.into()));

        return self;
    }
}

impl GepData {
    pub fn update(mut data: ResMut<Self>, mut commands: Commands) {
        for (cid, fns) in mem::take(&mut data.fn_deactivate) {
            if !data.prev_calls.contains(&cid) {
                for (entity, f) in fns {
                    f.0(&mut commands.entity(entity));
                }
            }
        }

        data.prev_calls = data.prev_calls_next.drain(..).collect();
        data.all_calls = data.all_calls_next.drain(..).collect();
    }
}

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub struct GepPlugin;

impl Plugin for GepPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GepData>();
        app.add_systems(Update, (GepData::update).in_set(GepPlugin));
    }
}
