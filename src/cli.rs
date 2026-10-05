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
    Exit,
}

pub fn parse(cmd: &str) -> Command {
    let args: Vec<&str> = cmd.split_whitespace().collect();

    match args.first().copied() {
        Some("add") => {
            let title = args.get(1).expect("Title is required");

            let priority = match args.iter().position(|arg| *arg == "--priority") {
                Some(index) => {
                    let value = args.get(index + 1).expect("Priority value is required");

                    match *value {
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
            let pending = args.contains(&"--pending");
            let done = args.contains(&"--done");
            if pending && done {
                panic!("Cannot use --pending and --done together");
            }

            let priority = match args.iter().position(|arg| *arg == "--priority") {
                Some(index) => {
                    let value = args.get(index + 1).expect("Priority value is required");

                    match *value {
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
            let query = args.get(1).expect("Query is required");

            Command::Search(query.to_string())
        }

        Some("stats") => Command::Stats,

        Some("exit") => Command::Exit,

        _ => panic!("Unknown command"),
    }
}
