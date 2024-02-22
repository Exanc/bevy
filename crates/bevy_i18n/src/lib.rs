//! TODO

mod locale;
mod localization;

/// Everything needed to localize your Bevy App
pub mod prelude {
    pub use crate::locale::*;
    pub use crate::localization::*;
}
