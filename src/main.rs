extern crate confy;

#[macro_use]
extern crate serde_derive;

pub mod cli;
pub use crate::cli::config::*;
pub use crate::cli::interface;

use anyhow::{Context, Result};
use log;

/// nextup CLI app
///
/// Allow the user to set up to 3 projects and update their 'nextup' tasks
fn main() -> Result<()> {
    // Init logger
    env_logger::init();
    log::debug!("Running nextup");

    // Load the current nextups
    let cfg: NextupConfig = confy::load("nextup-config", None)
        .with_context(|| format!("Could not read the saved nextup data."))?;
    log::info!("Loaded config: {:?}", &cfg);
    let mut projects: Vec<Project> = config_to_projects(cfg);
    let new_config: NextupConfig = NextupConfig::from(projects);

    // Get commands
    let matches = interface::commands().get_matches();

    match matches.subcommand() {
        Some(("reset", sub_matches)) => println!("'nextup reset' was called"),
        Some(("set", sub_matches)) => println!(
            "'nextup set' was called with project: {:?}, description: {:?}",
            sub_matches.get_one::<String>("project"),
            sub_matches.get_one::<String>("title"),
        ),
        _ => {
            if let Some(nextup) = matches.get_one::<String>("nextup") {
                println!(
                    "'nextup' was called with project: {:?}, nextup: {:?}",
                    matches.get_one::<String>("project"),
                    matches.get_one::<String>("nextup"),
                )
            } else if let Some(project) = matches.get_one::<String>("project") {
                println!(
                    "'nextup' was called with project: {:?}",
                    matches.get_one::<String>("project"),
                )
            } else {
                println!("'nextup' was called")
            }
        }
    }

    // Read or update nextups

    // Write any changes
    // confy::store("nextup-config", None, &new_cfg)
    //     .with_context(|| format!("Could not read the saved nextup data."))?;

    // log::debug!("Saved config: {:?}", &new_cfg);

    Ok(())
}

#[test]
fn verify_cmd() {
    cli().debug_assert();
}
