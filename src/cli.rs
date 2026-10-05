use std::env;

pub enum Command {
    Add(String),
    List { pending: bool, done: bool },
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

        _ => {
            println!("Usage:");
            println!("  task-cli add <task>");
            println!("  task-cli list");

            std::process::exit(1);
        }
    }
}
