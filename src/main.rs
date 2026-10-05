mod cli;
mod storage;
mod task;

use cli::Command;
use task::Task;

fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1
}

fn main() {
    let command = cli::parse();
    let mut tasks: Vec<Task> = storage::load();
    match command {
        Command::Add(title) => {
            let task = Task::new(next_id(&tasks), title);
            tasks.push(task);
            storage::sync(&tasks);
            println!("Task added");
        }
        Command::List { pending, done } => {
            if tasks.is_empty() {
                println!("No tasks found");
                return;
            }
            let filtered: Vec<&Task> = tasks
                .iter()
                .filter(|task| {
                    if pending {
                        !task.is_done
                    } else if done {
                        task.is_done
                    } else {
                        true
                    }
                })
                .collect();

            if filtered.is_empty() {
                if pending {
                    println!("No pending tasks");
                } else if done {
                    println!("No completed tasks");
                } else {
                    println!("No tasks");
                }
            }

            for task in filtered {
                println!(
                    "{}. {} [{}]",
                    task.id,
                    task.title,
                    if task.is_done { "done" } else { "pending" }
                );
            }
        }
    }
}
