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
    let cfg: NextupConfig = confy::load("nextup", None)
        .with_context(|| format!("Could not read the saved nextup data."))?;
    log::debug!("Loaded config: {:?}", &cfg);

    // Convert config data to project format
    let mut projects: Vec<Project> = config_to_projects(cfg);

    // Get commands
    let matches = interface::commands().get_matches();

    match matches.subcommand() {
        Some(("reset", _sub_matches)) => {
            log::debug!("'nextup reset' was called");
            for project in projects.iter_mut() {
                project.title = String::from(DEFAULT_FILL);
                project.nextup = String::from(DEFAULT_FILL);
            }
            println!("reset all projects");
        }
        Some(("set", sub_matches)) => {
            log::debug!(
                "'nextup set' was called with project: {:?}, description: {:?}",
                sub_matches.get_one::<String>("project"),
                sub_matches.get_one::<String>("title"),
            );
            let index: usize = map_project_id(sub_matches.get_one::<String>("project"));
            let set_project = &mut projects[index];
            set_project.title = sub_matches
                .get_one::<String>("title")
                .expect("Assumed safe to unwrap due to CLI checker")
                .clone();
            println!("{}", set_project);
        }
        _ => {
            if let Some(_nextup) = matches.get_one::<String>("nextup") {
                log::debug!(
                    "'nextup' was called with project: {:?}, nextup: {:?}",
                    matches.get_one::<String>("project"),
                    matches.get_one::<String>("nextup"),
                );
                let index: usize = map_project_id(matches.get_one::<String>("project"));
                let set_project = &mut projects[index];
                set_project.nextup = matches
                    .get_one::<String>("nextup")
                    .expect("Assumed safe to unwrap due to CLI checker")
                    .clone();
                println!("{}", set_project);
            } else if let Some(_project) = matches.get_one::<String>("project") {
                log::debug!(
                    "'nextup' was called with project: {:?}",
                    matches.get_one::<String>("project"),
                );
                let index: usize = map_project_id(matches.get_one::<String>("project"));
                println!("{}", projects[index]);
            } else {
                log::debug!("'nextup' was called");
                let lineup: Vec<&str> = vec!["a", "b", "c"];
                for (i, project) in projects.iter().enumerate() {
                    println!("{}: {}", lineup[i], project);
                }
            }
        }
    }

    // Convert project data to config format
    let upd_cfg: NextupConfig = NextupConfig::from(projects);

    // Save any nextup updates
    confy::store("nextup", None, &upd_cfg)
        .with_context(|| format!("Could not read the saved nextup data."))?;

    log::debug!("Saved config: {:?}", &upd_cfg);

    Ok(())
}

#[test]
fn verify_cmd() {
    cli().debug_assert();
}
