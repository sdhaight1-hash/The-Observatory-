use observatory_core::Observatory;
use std::{env, error::Error, process};

fn usage() -> &'static str {
    "Usage: observatory-core <database.sqlite> <command> [arguments]\n\
Commands:\n\
  new <research question>\n\
  claim <investigation-id> <claim|hypothesis> <statement>\n\
  source <investigation-id> <title> <reference>\n\
  link <claim-id> <source-id> <locator> <supports|contradicts|context> [note]\n\
  revise <source-id> <new-reference>\n\
  deps <source-id>\n\
  report <investigation-id>\n\
All source locations are supplied by users and are NOT verified."
}

fn id(value: &str) -> Result<i64, Box<dyn Error>> {
    Ok(value.parse::<i64>()?)
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(usage().into());
    }
    let mut store = Observatory::open(&args[1])?;
    let a = &args[2..];
    match a {
        [command, question] if command == "new" => {
            println!("{}", store.investigation(question)?);
        }
        [command, investigation, kind, statement] if command == "claim" => {
            println!("{}", store.claim(id(investigation)?, statement, kind)?);
        }
        [command, investigation, title, reference] if command == "source" => {
            println!("{}", store.source(id(investigation)?, title, reference)?);
        }
        [command, claim, source, locator, stance] if command == "link" => {
            println!("{}", store.link(id(claim)?, id(source)?, locator, stance, "")?);
        }
        [command, claim, source, locator, stance, note] if command == "link" => {
            println!("{}", store.link(id(claim)?, id(source)?, locator, stance, note)?);
        }
        [command, source, reference] if command == "revise" => {
            store.revise_source(id(source)?, reference)?;
            println!("Updated source");
        }
        [command, source] if command == "deps" => {
            for claim in store.dependent_claims(id(source)?)? {
                println!("{claim}");
            }
        }
        [command, investigation] if command == "report" => {
            print!("{}", store.report_markdown(id(investigation)?)?);
        }
        _ => return Err(usage().into()),
    };
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}
