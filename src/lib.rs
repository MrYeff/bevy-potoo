mod core;
mod drawable;
mod new;
mod new2;

use bevy::prelude::*;

use crate::core::{DrawNext, init_prim_meshes_materials, update_primitives, update_sprites};

pub struct PotooPlugin;

impl Plugin for PotooPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DrawNext>();
        app.add_systems(
            PostUpdate,
            ((update_sprites, update_primitives))
                .chain()
                .before(TransformSystems::Propagate),
        );
        app.add_systems(Startup, init_prim_meshes_materials);
    }
}

pub mod prelude {
    pub use crate::PotooPlugin;
    pub use crate::core::DrawNext;
    pub use crate::drawable::{Drawable, Primitive, PrimitiveShape};
}
