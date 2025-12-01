use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_potoo::prelude::*;

mod primitive;
mod sprite;

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        sprite::plugin(app);
        primitive::plugin(app);
    }
}

#[derive(SystemParam)]
pub struct Draw<'w, 's> {
    sprite: sprite::DrawSprite<'w, 's>,
    primitive: primitive::DrawPrimitive<'w, 's>,
}

impl Draw<'_, '_> {
    pub fn draw_sprite(
        &mut self,
        gep: &mut Gep,
        ident: Ident,
        sprite: impl Into<std::path::PathBuf>,
        iso: impl Into<Isometry2d>,
        scale: Vec2,
    ) {
        self.sprite.draw(gep, ident, sprite, iso, scale);
    }

    pub fn draw_primitive(
        &mut self,
        gep: &mut Gep,
        ident: Ident,
        primitive: impl Into<primitive::Primitive>,
        color: impl Into<Color>,
        iso: impl Into<Isometry2d>,
    ) {
        self.primitive.draw(gep, ident, primitive, color, iso);
    }
}

pub mod prelude {
    pub use crate::primitive::{DrawPrimitive, Primitive};
    pub use crate::sprite::DrawSprite;
    pub use crate::{Draw, DrawPlugin};
}
