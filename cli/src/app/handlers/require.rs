use crate::app::commands::RequireArgs;
use anyhow::Result;

pub fn require_handler(args: RequireArgs) -> Result<()> {
    dbg!(args);

    Ok(())
}
