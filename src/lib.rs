mod core;
mod fun;
mod ident;

pub mod prelude {
    pub use crate::core::*;
    pub use crate::fun::*;
    pub use crate::ident::*;
}
