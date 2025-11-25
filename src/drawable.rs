use bevy::prelude::*;

pub enum PrimitiveShape {
    Circle,
    Rect,
}

pub struct Primitive {
    pub color: Color,
    pub shape: PrimitiveShape,
}

pub enum Drawable {
    Image(Handle<Image>),
    Primitive(Primitive),
}
