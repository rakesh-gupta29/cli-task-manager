use core::panic;
use std::env;

use crate::task::Priority;

pub enum Command {
    Add(String, Priority),
    List {
        pending: bool,
        done: bool,
        priority: Option<Priority>,
    },
    Done(u32),
    Clear,
    Search(String),
    Stats,
}

pub fn parse() -> Command {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("add") => {
            let title = args.get(1).expect("Title must be a string").trim();

            if title.is_empty() {
                panic!("Title should not be empty");
            }

            let priority: Priority = match args.iter().position(|arg| arg == "--priority") {
                Some(index) => {
                    let value = args.get(index + 1).expect("Priority value is required");

                    match value.as_str() {
                        "high" => Priority::High,
                        "medium" => Priority::Medium,
                        "low" => Priority::Low,
                        _ => panic!("Priority must be low, medium, or high"),
                    }
                }
                None => Priority::Medium,
            };

            Command::Add(title.to_string(), priority)
        }

        Some("list") => {
            let pending = args.iter().any(|arg| arg == "--pending");
            let done = args.iter().any(|arg| arg == "--done");

            if pending && done {
                panic!("cannot use --pending and --done together");
            }

            let priority = match args.iter().position(|arg| arg == "--priority") {
                Some(index) => {
                    let value = args.get(index + 1).expect("Priority value is required");

                    match value.as_str() {
                        "low" => Some(Priority::Low),
                        "medium" => Some(Priority::Medium),
                        "high" => Some(Priority::High),
                        _ => panic!("Priority must be low, medium, or high"),
                    }
                }
                None => None,
            };

            Command::List {
                pending,
                done,
                priority,
            }
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

        Some("stats") => Command::Stats,

        _ => {
            println!("Usage:");
            println!("  task-cli add <task>");
            println!("  task-cli list");

            std::process::exit(1);
        }
    }
}
