use std::panic::Location;

use bevy::ecs::component::Component;

#[derive(Debug, Clone, Copy)]
pub enum Ident {
    /// Entity and Call only Identified by Loc
    Loc(Loc),
    /// Entity Only Identified by Key, Call identified by Loc and Key
    Keyed(Loc, Key),
    /// Entity and Call identified by Loc and Key
    LocKeyed(Loc, Key),
}

impl Ident {
    #[track_caller]
    pub fn here() -> Self {
        Self::Loc(Location::caller().into())
    }

    #[track_caller]
    pub fn keyed(key: impl Into<Key>) -> Self {
        Self::Keyed(Location::caller().into(), key.into())
    }

    #[track_caller]
    pub fn here_keyed(key: impl Into<Key>) -> Self {
        Self::LocKeyed(Location::caller().into(), key.into())
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Key {
    U64(u64),
    Str(&'static str),
}

impl From<u64> for Key {
    fn from(v: u64) -> Self {
        Key::U64(v)
    }
}
impl From<u32> for Key {
    fn from(v: u32) -> Self {
        Key::U64(v as u64)
    }
}
impl From<usize> for Key {
    fn from(v: usize) -> Self {
        Key::U64(v as u64)
    }
}

impl From<&'static str> for Key {
    fn from(v: &'static str) -> Self {
        Key::Str(v)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Loc(u64);

impl From<&'static Location<'_>> for Loc {
    fn from(v: &'static Location<'_>) -> Self {
        let callsite = (v as *const Location) as u64;
        Loc(callsite)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub(crate) enum CallsiteIdent {
    LocOnly(Loc),
    WithKey(Loc, Key),
}

impl From<Ident> for CallsiteIdent {
    fn from(value: Ident) -> Self {
        match value {
            Ident::Loc(loc) => Self::LocOnly(loc),
            Ident::Keyed(loc, key) | Ident::LocKeyed(loc, key) => Self::WithKey(loc, key),
        }
    }
}

#[derive(Component, Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub(crate) enum EntityIdent {
    LocOnly(Loc),
    WithKey(Loc, Key),
    Global(Key),
}

impl From<Ident> for EntityIdent {
    fn from(value: Ident) -> Self {
        match value {
            Ident::Loc(loc) => Self::LocOnly(loc),
            Ident::Keyed(_, key) => Self::Global(key),
            Ident::LocKeyed(loc, key) => Self::WithKey(loc, key),
        }
    }
}
