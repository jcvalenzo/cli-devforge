pub mod banner;
pub mod cli;
pub mod commands;
pub mod error;
pub mod output;
pub mod prefix;
pub mod scaffold;
pub mod template;

pub use error::{Error, Result};

pub const VERSION: &str = env!("DEVFORGE_VERSION");
