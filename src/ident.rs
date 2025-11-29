use std::panic::Location;

use bevy::ecs::component::Component;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Loc(u64);

impl Loc {
    #[track_caller]
    pub fn here() -> Self {
        let loc: &'static Location = Location::caller();
        let callsite = (loc as *const Location) as u64;
        Loc(callsite)
    }
}

impl From<&'static Location<'_>> for Loc {
    fn from(v: &'static Location<'_>) -> Self {
        let callsite = (v as *const Location) as u64;
        Loc(callsite)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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

#[derive(Component, Hash, Eq, PartialEq, Debug, Clone)]
pub enum Ident {
    Loc(Loc),
    Key(Key),
    LocAndKey(Loc, Key),
}

impl<T: Into<Key>> From<T> for Ident {
    fn from(v: T) -> Self {
        Ident::Key(v.into())
    }
}

impl From<Loc> for Ident {
    fn from(v: Loc) -> Self {
        Ident::Loc(v)
    }
}

impl<T: Into<Key>> From<(Loc, T)> for Ident {
    fn from(v: (Loc, T)) -> Self {
        Ident::LocAndKey(v.0, v.1.into())
    }
}
