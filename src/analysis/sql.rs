use anyhow::{Context, Error, Result};
use turbosql::{Turbosql, select, execute};

#[derive(Turbosql, Default)]
struct Project {
    rowid: Option<i64>, // rowid member required & enforced at compile time
    slot: Option<String>,
    title: Option<String>,
    timestamp: Option<uszie>, // Convert to chrono
}


#[derive(Turbosql, Default)]
struct Audit {
    rowid: Option<i64>, // rowid member required & enforced at compile time
    timestamp: Option<uszie>, // Convert to chrono
    projectid: Option<i64>,
    nextup: Option<String>,
}

/// Initialize the sqllite db
pub fn log_audit() -> Result<(), Error>  {
    // EXAMPLE
    let name = "Joe";
    // INSERT a row
    let rowid = Person {
        name: Some(name.to_string()),
        age: Some(42),
        ..Default::default()
    }.insert()?;

    // SELECT all rows
    let people = select!(Vec<Person>)?;

    // SELECT multiple rows with a predicate
    let people = select!(Vec<Person> "WHERE age > " 21)?;

    // SELECT a single row with a predicate
    let mut person = select!(Person "WHERE name = " name)?;

    // UPDATE based on rowid, rewrites all fields in database row
    person.age = Some(43);
    person.update()?;

    // UPDATE with manual SQL
    execute!("UPDATE person SET age = " 44 " WHERE name = " name)?;

    // DELETE
    execute!("DELETE FROM person WHERE rowid = " 1)?;
    // EXAMPLE

    Ok(())
}