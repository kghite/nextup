use crate::{map_project_id, Project, DEFAULT_FILL};
use clap::ArgMatches;

/// Reset all projects to default value
pub fn reset_to_defaults(projects: &mut Vec<Project>) {
    for project in projects.iter_mut() {
        project.title = String::from(DEFAULT_FILL);
        project.nextup = String::from(DEFAULT_FILL);
    }

    println!("reset all projects");
}

/// Write the title to a given project id
pub fn write_title(projects: &mut Vec<Project>, sub_matches: &ArgMatches) {
    let index: usize = map_project_id(sub_matches.get_one::<String>("project"));
    let set_project = &mut projects[index];
    set_project.title = sub_matches
        .get_one::<String>("title")
        .expect("Assumed safe to unwrap due to CLI checker")
        .clone();

    println!("{}", set_project);
}

/// Write the nextup to a given project id
pub fn write_nextup(projects: &mut Vec<Project>, matches: ArgMatches) {
    let index: usize = map_project_id(matches.get_one::<String>("project"));
    let set_project = &mut projects[index];
    set_project.nextup = matches
        .get_one::<String>("nextup")
        .expect("Assumed safe to unwrap due to CLI checker")
        .clone();

    println!("{}", set_project);
}

/// Display a project
pub fn report_project(projects: &mut Vec<Project>, matches: ArgMatches) {
    let index: usize = map_project_id(matches.get_one::<String>("project"));

    println!("{}", projects[index])
}

/// Display all projects
pub fn report_projects(projects: &mut Vec<Project>) {
    let lineup: Vec<&str> = vec!["a", "b", "c"];
    for (i, project) in projects.iter().enumerate() {
        println!("{}: {}", lineup[i], project);
    }
}
