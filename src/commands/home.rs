use crate::banner;
use crate::Result;

pub fn run() -> Result<()> {
    banner::print_home();
    Ok(())
}
