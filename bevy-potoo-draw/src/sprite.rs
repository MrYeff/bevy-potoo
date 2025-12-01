use bevy::ecs::system::SystemParam;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_potoo::prelude::*;
use std::path::PathBuf;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Ctx>();
}

#[derive(SystemParam)]
pub struct DrawSprite<'w, 's> {
    ctx: ResMut<'w, Ctx>,
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
}

#[derive(Resource, Default)]
struct Ctx {
    images: HashMap<PathBuf, Handle<Image>>,
}

impl DrawSprite<'_, '_> {
    pub fn draw(
        &mut self,
        gep: &mut Gep,
        ident: Ident,
        sprite: impl Into<PathBuf>,
        iso: impl Into<Isometry2d>,
        scale: Vec2,
    ) {
        let sprite = sprite.into();
        let iso = iso.into();

        let e = gep.get(ident);

        if gep.when_once(ident) {
            let image = self
                .ctx
                .images
                .entry(sprite.clone())
                .or_insert_with(|| self.asset_server.load(sprite))
                .clone();

            self.commands.entity(e).insert(Sprite::from_image(image));
        }

        if gep.when_activate(ident) {
            self.commands.entity(e).insert(Visibility::Visible);
        }

        self.commands.entity(e).insert(Transform {
            translation: Vec3::new(iso.translation.x, iso.translation.y, 0.0),
            rotation: Quat::from_rotation_z(iso.rotation.as_radians()),
            scale: scale.extend(1.0),
        });

        gep.on_deactivate(ident, |ec: &mut EntityCommands| {
            ec.insert(Visibility::Hidden);
        });
    }
}
