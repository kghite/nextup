#[derive(Debug, Serialize, Deserialize)]
pub struct NextupConfig {
    a: String,
    b: String,
    c: String,
}

/// `Config` implements `Default`
impl Default for NextupConfig {
    fn default() -> Self {
        NextupConfig {
            a: "project,nextup".to_string(),
            b: "project,nextup".to_string(),
            c: "project,nextup".to_string(),
        }
    }
}

/// Config implements from(Vec<Project>)
impl From<Vec<Project>> for NextupConfig {
    fn from(projects: Vec<Project>) -> Self {
        NextupConfig {
            a: format!("{},{}", projects[0].title, projects[0].nextup),
            b: format!("{},{}", projects[0].title, projects[0].nextup),
            c: format!("{},{}", projects[0].title, projects[0].nextup),
        }
    }
}

#[derive(Debug)]
pub struct Project {
    title: String,
    nextup: String,
}

/// Implement from(NextupConfig) alternative for Vec<Project>
pub fn config_to_projects(config: NextupConfig) -> Vec<Project> {
    let projects: Vec<Project> = [config.a, config.b, config.c]
        .into_iter()
        .map(|pair| {
            let split: Vec<&str> = pair.split(",").collect();
            Project {
                title: split[0].to_string(),
                nextup: split[1].to_string(),
            }
        })
        .collect();

    projects
}
