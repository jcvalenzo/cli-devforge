use crate::{Result, VERSION};

pub fn run() -> Result<()> {
    println!("devforge {VERSION}");
    Ok(())
}
