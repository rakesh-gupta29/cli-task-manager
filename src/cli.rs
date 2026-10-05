use core::panic;
use std::env;

pub enum Command {
    Add(String),
    List { pending: bool, done: bool },
    Done(u32),
    Clear,
    Search(String),
}

pub fn parse() -> Command {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("add") => {
            let title = args.get(1).expect("Title must be a string").trim();

            if title.is_empty() {
                panic!("Title should not be empty");
            }

            Command::Add(title.to_string())
        }

        Some("list") => {
            let pending = args.iter().any(|elem| elem == "--pending");
            let done = args.iter().any(|arg| arg == "--done");
            if pending && done {
                panic!("cannot use --pending and --done together");
            }
            Command::List { pending, done }
        }
        Some("done") => {
            let id = args
                .get(1)
                .expect("Id is required")
                .parse::<u32>()
                .expect("Id should be a valid integer");

            Command::Done(id)
        }
        Some("clear") => Command::Clear,
        Some("search") => {
            let query = args.get(1).expect("Query should be a valid string").trim();

            Command::Search(query.to_string())
        }

        _ => {
            println!("Usage:");
            println!("  task-cli add <task>");
            println!("  task-cli list");

            std::process::exit(1);
        }
    }
}
