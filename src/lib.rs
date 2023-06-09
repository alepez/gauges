#[cfg(feature = "desktop")]
pub mod desktop;

pub mod app;
pub mod core;
pub mod net;

pub mod prelude {
    pub use crate::core::*;
}
