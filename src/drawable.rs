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

impl From<Handle<Image>> for Drawable {
    fn from(img: Handle<Image>) -> Self {
        Drawable::Image(img)
    }
}

impl From<Primitive> for Drawable {
    fn from(prim: Primitive) -> Self {
        Drawable::Primitive(prim)
    }
}

impl Primitive {
    pub fn circle(color: Color) -> Self {
        Primitive {
            color,
            shape: PrimitiveShape::Circle,
        }
    }

    pub fn rect(color: Color) -> Self {
        Primitive {
            color,
            shape: PrimitiveShape::Rect,
        }
    }
}
