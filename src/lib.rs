pub use bevy::prelude::*;

macro_rules! expose_submodules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            pub use self::$x::*;
        )*
    };
}

pub mod camera;
pub mod input;
pub mod map;
pub mod settings;
pub mod utils;

pub use crate::camera::*;
pub use crate::input::*;
pub use crate::map::*;
pub use crate::settings::*;
pub use crate::utils::*;
