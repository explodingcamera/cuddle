use crate::CuddleApp;
use anyhow::{Context, Result};

use dialoguer::console::Style;
use dialoguer::{theme::ColorfulTheme, Input};

pub fn run(app: CuddleApp) -> Result<()> {
    let mut cfg = (app.get_config)()?;

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    println!("Due to restrictions in the code api, please paste you refresh token (cid) below.\n");
    println!("Important note: \nFor now, cid's are stored in the config file, so be sure to exclude `~/.config/cuddle` if your using some kind of dotfiles manager.");
    println!("To find your cid:");
    println!("1. Go to https://app.code.berlin and sign in");
    println!("2. Go to https://api.app.code.berlin");
    println!("3. Open the Developer Console (F12)");
    println!("4. Go to Storage (Firefox) / Applications (Chrome)");
    println!("5. Copy the value of the cookie named cid and marked as http-only");
    println!("    (The cid should be 3 long strings seperated by dots)");
    println!("6. Paste the value (SHIFT+INSERT or CTRL+Shift+V depending on the terminal)");

    cfg.cid = Input::with_theme(&theme)
        .with_prompt("Please paste your CID")
        .interact()
        .with_context(|| format!("Failed to get cid"))?;

    (app.set_config)(cfg)?;

    println!("Successfully saved cid to config!");

    Ok(())
}
