use parsing::lex;
use std::{error::Error, path::PathBuf};

pub mod parsing;

mod jar;
pub use jar::*;

#[cfg(test)]
pub mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let db = Database::new();
    let initial = db.input(Into::<PathBuf>::into("workspace.kc"))?;

    let res = lex(&db, initial);
    
    for err in res.errors(&db)
    {
        println!("{}", err.reason());
    }

    for (token, _) in res.contents(&db) 
    {
        print!("{} ", token);
    }

    Ok(())
}
