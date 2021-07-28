use crate::CuddleApp;
use anyhow::Result;

pub fn run(app: CuddleApp) -> Result<()> {
    let _cfg = (app.get_config)()?;
    Ok(())
}
