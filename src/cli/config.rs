use std::fmt;

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
            b: format!("{},{}", projects[1].title, projects[1].nextup),
            c: format!("{},{}", projects[2].title, projects[2].nextup),
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub title: String,
    pub nextup: String,
}

/// Format the project display to usage spec
impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n\t\x1b[94mnextup\x1b[0m: {}\n",
            self.title, self.nextup
        )
    }
}

pub const DEFAULT_FILL: &str = "<not set>";

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

pub fn map_project_id(id: Option<&String>) -> usize {
    let lineup: Vec<&str> = vec!["a", "b", "c"];
    let index = lineup
        .iter()
        .position(|&x| {
            x == id
                .expect("Assumed safe to unwrap due to CLI checker")
                .as_str()
        })
        .unwrap();
    index
}
