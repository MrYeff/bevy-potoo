use bevy::prelude::*;

pub struct EntityFn(pub(crate) Box<dyn FnOnce(&mut EntityCommands) + Send + Sync + 'static>);

impl<T: FnOnce(&mut EntityCommands) + Send + Sync + 'static> From<T> for EntityFn {
    fn from(v: T) -> Self {
        Self(Box::new(v))
    }
}
