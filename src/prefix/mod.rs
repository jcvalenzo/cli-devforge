mod detect;
mod registry;

pub use detect::detect;
pub use registry::{all, find, Prefix};
