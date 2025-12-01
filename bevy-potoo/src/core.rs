use crate::fun::TreadSafeEntityFn;
use crate::ident::{CallsiteIdent, EntityIdent, Ident};
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::{ecs::system::SystemParam, platform::collections::HashMap};
use std::fmt::Debug;
use std::mem;

#[derive(Resource, Default)]
struct GepData {
    ident_to_entity: HashMap<EntityIdent, Entity>,

    all_calls: HashSet<CallsiteIdent>,
    new_calls: HashSet<CallsiteIdent>,

    prev_calls: HashSet<CallsiteIdent>,
    prev_calls_next: Vec<CallsiteIdent>,

    fn_deactivate: HashMap<CallsiteIdent, Vec<(Entity, TreadSafeEntityFn)>>,
    fn_deactivate_next: HashMap<CallsiteIdent, Vec<(Entity, TreadSafeEntityFn)>>,
}

/// Global Entity Protocol
#[derive(SystemParam)]
pub struct Gep<'w, 's> {
    data: ResMut<'w, GepData>,
    commands: Commands<'w, 's>,
}

impl<'w, 's> Gep<'w, 's> {
    fn handle_callsite(&mut self, ident: Ident) -> CallsiteIdent {
        let callsite_ident: CallsiteIdent = ident.into();
        self.data.prev_calls_next.push(callsite_ident);
        if !self.data.all_calls.contains(&callsite_ident) {
            self.data.all_calls.insert(callsite_ident);
            self.data.new_calls.insert(callsite_ident);
        }
        callsite_ident
    }

    pub fn get(&mut self, ident: Ident) -> Entity {
        self.handle_callsite(ident);

        let entity_ident = ident.into();
        if let Some(entity) = self.data.ident_to_entity.get(&entity_ident) {
            return *entity;
        }

        let entity = self.commands.spawn(entity_ident).id();
        self.data.ident_to_entity.insert(entity_ident, entity);

        entity
    }

    pub fn when_once(&mut self, ident: Ident) -> bool {
        self.handle_callsite(ident);

        let callsite_ident: CallsiteIdent = ident.into();
        self.data.new_calls.contains(&callsite_ident)
    }

    pub fn when_activate(&mut self, ident: Ident) -> bool {
        self.handle_callsite(ident);

        let callsite_ident: CallsiteIdent = ident.into();
        !self.data.prev_calls.contains(&callsite_ident)
    }

    pub fn on_deactivate(&mut self, ident: Ident, f: impl Into<TreadSafeEntityFn>) {
        self.handle_callsite(ident);

        let callsite_ident: CallsiteIdent = ident.into();
        let entity_ident: EntityIdent = ident.into();
        if let Some(entity) = self.data.ident_to_entity.get(&entity_ident).cloned() {
            self.data
                .fn_deactivate_next
                .entry(callsite_ident)
                .or_default()
                .push((entity, f.into()));
        }
    }
}
impl GepData {
    pub fn update(mut data: ResMut<Self>, mut commands: Commands) {
        let fn_deactivate_next = mem::take(&mut data.fn_deactivate_next);
        for (cid, fns) in mem::replace(&mut data.fn_deactivate, fn_deactivate_next) {
            if !data.prev_calls_next.contains(&cid) {
                for (entity, f) in fns {
                    f.0(&mut commands.entity(entity));
                }
            }
        }

        data.prev_calls = data.prev_calls_next.drain(..).collect();
        data.new_calls.clear();
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
