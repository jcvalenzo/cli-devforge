use crate::output;
use crate::prefix;
use crate::Result;

pub fn run() -> Result<()> {
    output::print_list(prefix::all());
    Ok(())
}
