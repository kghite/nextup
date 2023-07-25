extern crate confy;

#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use log;

pub mod cli;

pub use crate::cli::config::*;
pub use crate::cli::interface;
pub use crate::cli::writer;

/// nextup CLI app
///
/// Allow the user to set up to 3 projects and update their 'nextup' tasks
fn main() -> Result<()> {
    let app_name: String = format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    // Init logger
    env_logger::init();
    log::debug!("Running nextup");

    // Load the current projects
    let mut projects: Vec<Project> = load_projects(app_name.as_str())?;

    // Map commands
    let matches = interface::commands().get_matches();
    match matches.subcommand() {
        // Reset all projects and nextups
        Some(("reset", _sub_matches)) => {
            writer::reset_to_defaults(&mut projects);
        }
        // Set a project title
        Some(("set", sub_matches)) => {
            writer::write_title(&mut projects, sub_matches);
        }
        _ => {
            // Set a project's nextup
            if let Some(_nextup) = matches.get_one::<String>("nextup") {
                writer::write_nextup(&mut projects, matches);
            }
            // Show a project
            else if let Some(_project) = matches.get_one::<String>("project") {
                writer::report_project(&mut projects, matches);
            }
            // Show all projects
            else {
                writer::report_projects(&mut projects);
            }
        }
    }

    // Save the updated projects
    save_projects(app_name.as_str(), projects)?;

    Ok(())
}

#[test]
fn verify_cmd() {
    interface::commands().debug_assert();
}
