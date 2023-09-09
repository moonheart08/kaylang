use chumsky::prelude::*;
use std::{fs, error::Error};

pub mod parsing;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("workspace.kc")?;

    let (t, errs) = parsing::lexer().parse(file.as_str()).into_output_errors();
    
    for err in errs 
    {
        println!("{}", err.reason());
    }

    if let Some(tokens) = t 
    {
        for (token, _) in tokens {
            print!("{} ", token);
        }
    }


    Ok(())
}
