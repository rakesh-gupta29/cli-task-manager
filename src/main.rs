mod cli;
mod storage;
mod task;

use cli::Command;
use task::Priority;
use task::Stats;
use task::Task;

fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1
}

fn main() {
    let command = cli::parse();
    let mut tasks: Vec<Task> = storage::load();
    match command {
        Command::Add(title, priority) => {
            let task = Task::new(next_id(&tasks), title, priority);
            tasks.push(task);
            storage::sync(&tasks);
            println!("Task added");
        }
        Command::List {
            pending,
            done,
            priority,
        } => {
            if tasks.is_empty() {
                println!("No tasks found");
                return;
            }
            let mut filtered: Vec<&Task> = tasks
                .iter()
                .filter(|task| {
                    if pending && task.is_done {
                        return false;
                    }

                    if done && !task.is_done {
                        return false;
                    }
                    match &priority {
                        Some(priority) => {
                            if &task.priority != priority {
                                return false;
                            }
                        }
                        None => {}
                    }

                    true
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

            filtered.sort_by_key(|task| std::cmp::Reverse(task.priority as u8));
            pretty_print(&filtered)
        }

        Command::Done(id) => match tasks.iter_mut().find(|elem| elem.id == id) {
            Some(elem) => {
                if elem.is_done {
                    println!("Task {} is already done", id);
                    return;
                }

                elem.is_done = true;
                storage::sync(&tasks);
                println!("Task {} marked as done", id);
            }

            None => println!("No matching task found"),
        },
        Command::Search(query) => {
            let filtered: Vec<&Task> = tasks
                .iter()
                .filter(|elem| elem.title.to_lowercase().contains(&query))
                .collect();
            if filtered.is_empty() {
                println!("No results found");
            } else {
                pretty_print(&filtered);
            }
        }

        Command::Stats => {
            let mut pending = 0;
            let mut done = 0;
            let mut high = 0;
            let mut medium = 0;
            let mut low = 0;

            for task in &tasks {
                match task.is_done {
                    true => done += 1,
                    false => pending += 1,
                }

                match task.priority {
                    Priority::High => high += 1,
                    Priority::Medium => medium += 1,
                    Priority::Low => low += 1,
                }
            }

            let stats = Stats {
                total: tasks.len(),
                pending,
                completed: done,
                high,
                medium,
                low,
            };
            print_stats(&stats);
        }
        Command::Clear => {
            tasks.clear();
            storage::sync(&tasks);
        }
    }
}

fn pretty_print(tasks: &[&Task]) {
    for task in tasks {
        let status = if task.is_done { "✅" } else { "⏳" };
        println!("{} {} [{:?}]", status, task.title, task.priority);
    }
}

fn print_stats(stats: &Stats) {
    println!("📊 Task Statistics\n");
    println!("Total:      {}", stats.total);
    println!("Pending:    {}", stats.pending);
    println!("Completed:  {}", stats.completed);
    println!();
    println!("Priority:");
    println!("🔴 High:    {}", stats.high);
    println!("🟡 Medium:  {}", stats.medium);
    println!("🟢 Low:     {}", stats.low);
}
