use anyhow::{Context, Error, Result};
use turbosql::{Turbosql, select, execute};

trait dbLog{}

#[derive(Turbosql, Default)]
struct ProjectLog {
    rowid: Option<i64>, // rowid member required & enforced at compile time
    slot: Option<String>,
    title: Option<String>,
    timestamp: Option<uszie>, // Convert to chrono
}

impl dbLog for ProjectLog{}

#[derive(Turbosql, Default)]
struct AuditLog {
    rowid: Option<i64>, // rowid member required & enforced at compile time
    timestamp: Option<uszie>, // Convert to chrono
    projectid: Option<i64>,
    nextup: Option<String>,
}

impl dbLog for AuditLog{}

/// Log nextup write actions for analysis
pub fn log_write_action<Loggable:dbLog>(action: &mut Loggable) -> Result<(), Error>  {
    let rowid = action.insert().with_context(|| format!("Could not save action to the db."))?;
    Ok(())
}
