use super::TabsState;
use crate::{Config, CuddleApp};
use anyhow::Result;

pub struct Dashboard<'a> {
    pub app: CuddleApp<'a>,
    pub config: Config,
    pub tabs: TabsState<'a>,
    pub title: &'a str,
}

impl<'a> Dashboard<'a> {
    pub fn new(app: CuddleApp<'a>) -> Result<Dashboard<'a>> {
        let config = (app.get_config)()?;
        Ok(Dashboard {
            app,
            title: "cuddle - code university dashboard",
            config: config,
            tabs: TabsState::new(vec!["Home", "Search", "Profile"]),
        })
    }
}
