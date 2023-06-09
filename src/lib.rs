#[cfg(feature = "desktop")]
pub mod desktop;

pub mod app;
pub mod core;

#[cfg(feature = "net")]
pub mod net;

pub mod prelude {
    pub use crate::core::*;
}
