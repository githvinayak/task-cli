// Day 1-2 - Ownership in Rust
// struct Task {
//     id: i32,
//     title: String,
//     done: bool,
// }

// fn main() {
//     // integers
//     let task1 = Task {
//         id: 1,
//         title: String::from("Learn Rust"),
//         done: false,
//     };
//     let x: i32 = 5;
//     let y = x;
//     println!("x:{} y:{}", x, y);

//     // floats
//     let a: f64 = 3.14;
//     let b = a;
//     println!("a:{} b:{}", a, b);

//     // bool
//     let t: bool = true;
//     let f = t;
//     println!("t:{} f:{}", t, f);

//     // char
//     let c1: char = 'R';
//     let c2 = c1;
//     println!("c1:{} c2:{}", c1, c2);

//     // now String
//     let s1 = String::from("hello");
//     let _s2 = s1;
//     // println!("s1:{}", s1); // what happens here?
//     println!(
//         "task id:{} and task title:{} and task status:{}",
//         task1.id, task1.title, task1.done
//     )
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_task_creation() {
//         let task = Task {
//             id: 1,
//             title: String::from("Learn Rust"),
//             done: false,
//         };
//         assert_eq!(task.id, 1);
//         assert_eq!(task.title, "Learn Rustt");
//         assert_eq!(task.done, false);
//     }
// }

// Day 3 - Borrowing and References in Rust

// #[derive(Debug)]
// struct Task {
//     id: u32,
//     title: String,
//     done: bool,
// }

// fn print_task(task:&Task){
//     println!("id:{} title:{} done:{}", task.id, task.title, task.done);
// }

// fn mar_down(task:&mut Task){
//     task.done = true
// }
// fn main(){
//  let mut task1 = Task{
//     id:1,
//     title:String::from("Learn Borrowing"),
//     done:false
//  };
//  print_task(&task1);
//  mar_down(&mut task1);
//  println!("task title:{}",task1.done);
//  println!("task1 :{:#?}",task1)
// }

// fn main() {
//     let mut task1 = Task {
//         id: 1,
//         title: String::from("Learn Rust"),
//         done: false,
//     };

//     let r1 = &task1;      // immutable borrow
//     let r2 = &task1;      // another immutable borrow
//     let r3 = &mut task1;  // mutable borrow

//     println!("{} {} {}", r1.title, r2.title, r3.title);
// }

// fn add_task(tasks: &mut Vec<Task>, id: u32, title: String, done: bool) {
//     let task = Task {
//         id: id,
//         title: title,
//         done: done,
//     };
//     tasks.push(task)
// }

// fn main() {
//     let mut tasks: Vec<Task> = Vec::new();
//     add_task(&mut tasks, 1, String::from("leran rust"), false);
//     add_task(&mut tasks, 1, String::from("leran rust"), false);
//     add_task(&mut tasks, 1, String::from("leran rust"), false);

//     for task in &tasks {
//         println!("task : {:#?}", task)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_add_task() {
//         let mut tasks: Vec<Task> = Vec::new();
//         add_task(&mut tasks, 1, String::from("learn rust"), false);
//         assert_eq!(tasks.len(), 1);
//         assert_eq!(tasks[0].id, 1);
//         assert_eq!(tasks[0].title, "learn rust");
//     }
// }

// fn print_str(str:&String){
//     println!("my name is :{}",str);
// }
// struct Task {
//     id: u32,
//     title: String,
//     done: bool, }
//

// fn add_task(tasks: &mut Vec<Task>, id: u32, title: String, done: bool) {
//     let task = Task {
//         id: id,
//         title: title,
//         done: done,
//     };
//     tasks.push(task)
// }

// fn list_tasks(tasks: &[Task]){
//      for task in tasks {
//         println!("task id: {} | task title: {} | task status: {}", [task.id](http://task.id/),task.title,task.done)
//     }
// }

// fn main() {
//     let mut tasks: Vec<Task> = Vec::new();
//     add_task(&mut tasks, 1, String::from("leran rust"), false);
//     add_task(&mut tasks, 1, String::from("leran rust"), false);
//     add_task(&mut tasks, 1, String::from("leran rust"), false);
//     list_tasks(&tasks);
// }

// #[cfg(test)]
// mod tests{
//  use super::*;
//     #[test]
//     fn test_list_is_empty_initially() {
//     let mut tasks : Vec<Task> = Vec::new();
//         assert_eq!(tasks.is_empty(),true)
//     }

//     #[test]
//     fn test_list_has_correct_count() {
//     let mut tasks : Vec<Task> = Vec::new();
//          add_task(&mut tasks, 1, String::from("learn rust"), false);
//           add_task(&mut tasks, 2, String::from("learn js"), false);
//           assert_eq!(tasks.len(),2)
//     }

//     #[test]
//     fn test_task_fields_are_correct() {
//     let mut tasks : Vec<Task> = Vec::new();
//          add_task(&mut tasks, 1, String::from("learn rust"), false);
//           assert_eq!(tasks[0].title,"learn rust")
//     }
// }
mod command;
mod storage;
mod task;
use command::{Command, SortOrder, parse_command};
use std::env;
use std::io;
use std::io::Write;
use storage::{load_tasks, save_tasks};
use task::Task;

// check if file exists
//std::path::Path::new(FILE_PATH).exists()

// // read file to string
// std::fs::read_to_string(FILE_PATH).unwrap()

// // parse JSON string to Vec<Task>
// serde_json::from_str(&content).unwrap()

// // convert Vec<Task> to JSON string (pretty printed)
// serde_json::to_string_pretty(tasks).unwrap()

// // write string to file
// std::fs::write(FILE_PATH, content).unwrap()

// fn list_tasks(tasks: &[Task]) {
//    println!("Your Tasks..");
//    println!("{}","_".repeat(30));
//    println!("\n Pending");
//    tasks.iter().filter(|t| !t.done).for_each(|t| println!("{}.{}",t.id,t.title));
//    println!("\n Completed");
//    tasks.iter().filter(|t| t.done).for_each(|t| println!("{}.{}",t.id,t.title));
//     println!("{}","_".repeat(30));
// println!("Task completed :{}",tasks.iter().filter(|t| !t.done).count());
// println!("Task pending :{}",tasks.iter().filter(|t| t.done).count())
// }

// fn list_tasks(tasks: &[Task],sort:SortOrder) {
//     println!("📋 Your Tasks");
//     println!("{}", "─".repeat(30));

//     println!("\n⏳ Pending:");
//     tasks
//         .iter()
//         .filter(|t| !t.done)
//         .for_each(|t| println!("  {} . {}", t.id, t.title));

//     println!("\n✅ Completed:");
//     tasks
//         .iter()
//         .filter(|t| t.done)
//         .for_each(|t| println!("  {}. {}", t.id, t.title));

//     println!("{}", "─".repeat(30));
//     println!("✅ Completed : {}", tasks.iter().filter(|t| t.done).count());
//     println!(
//         "⏳ Pending   : {}",
//         tasks.iter().filter(|t| !t.done).count()
//     );
// }

fn list_tasks(tasks: &[Task], sort: SortOrder) {
    let mut sorted = tasks.to_vec();
    // println!("📋 Your Tasks{:?} and order of sorting {:?}", &tasks, &sort);
    match sort {
        SortOrder::SortById => {
            sorted.sort_by(|a, b| a.id.cmp(&b.id));
        }
        SortOrder::SortByStatus => {
            sorted.sort_by(|a, b| a.done.cmp(&b.done));
        }
    }

    println!("📋 Your Tasks");
    println!("{}", "─".repeat(30));

    println!("\n⏳ Pending:");
    sorted
        .iter()
        .filter(|t| !t.done)
        .for_each(|t| println!("  {} . {}", t.id, t.title));

    println!("\n✅ Completed:");
    sorted
        .iter()
        .filter(|t| t.done)
        .for_each(|t| println!("  {}. {}", t.id, t.title));

    println!("{}", "─".repeat(30));
    println!(
        "✅ Completed : {}",
        sorted.iter().filter(|t| t.done).count()
    );
    println!(
        "⏳ Pending   : {}",
        sorted.iter().filter(|t| !t.done).count()
    );
}

fn find_task(tasks: &mut [Task], id: u32) -> Option<&mut Task> {
    tasks.iter_mut().find(|t| t.id == id)
}

// fn pending_tasks(tasks: &[Task]) -> usize {
//     tasks.iter().filter(|t| !t.done).count()
// }

// fn all_done(tasks: &[Task]) -> bool {
//     tasks.iter().all(|t| t.done)
// }
fn validate_title(title: &str) -> bool {
    !title.trim().is_empty()
}

fn ask_confirmation(message: &str) -> bool {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input: String = String::from("");
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim() == "yes",
        Err(err) => {
            println!("can;t read user input {}", err);
            false
        }
    }
}

fn run_command(cmd: Command, tasks: &mut Vec<Task>) {
    // println!("tasks {}",tasks.len());
    match cmd {
        Command::Add(title) => {
            let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            if validate_title(&title) {
                tasks.push(Task::new(id, title.clone()));
                println!("Task {} added : {}", id, title);
            } else {
                println!("Please provide valid title")
            }
        }
        Command::List(sort) => list_tasks(tasks, sort),
        Command::Done(id) => match find_task(tasks, id) {
            Some(task) => {
                task.mark_done();
                println!("Task {} marked as done", id);
            }
            None => println!("❌ No task found with id: {}", id),
        },
        Command::Delete(id) => {
            if tasks.iter().any(|t| t.id == id) {
                tasks.retain(|t| t.id != id);
                println!("Task {} deleted", id);
            } else {
                println!("❌ No task found with id: {}", id);
            }
        }
        Command::Clear => {
            if ask_confirmation("Arey you sure yes/no?") {
                tasks.clear();
            } else {
                println!("cancelled")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
 println!("args: {:?}", args);
    if args.len() < 2 {
        println!("Usage: taskcli <command> [args]");
        println!("Commands: add <title> | list <sorting order>| done <id> | delete <id>");
        return;
    }

    let input: String = args[1..].join(" ");
    let mut tasks = load_tasks();
    // println!("DEBUG after load: {} tasks", tasks.len()); // ← add this

    match parse_command(&input) {
        Some(cmd) => run_command(cmd, &mut tasks),
        None => println!("invalid command: {}", input),
    }

    // println!("DEBUG before save: {} tasks", tasks.len()); // ← add this
    save_tasks(&tasks);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_tasks() -> Vec<Task> {
        vec![
            Task::new(1, String::from("Buy milk")),
            Task::new(2, String::from("Learn Rust")),
        ]
    }

    #[test]
    fn test_add_task() {
        // create empty vec
        let mut tasks: Vec<Task> = Vec::new();
        // run_command(Command::Add(...), &mut tasks)\
        let title: String = "Buy milk".to_string();
        run_command(Command::Add(title), &mut tasks);
        // check tasks.len() == 1
        assert_eq!(tasks.len(), 1);
        // check title is correct
        assert_eq!(tasks[0].title, "Buy milk")
    }

    #[test]
    fn test_list_tasks() {
        // use setup_tasks()
        let tasks: Vec<Task> = setup_tasks();
        // check len is 2
        assert_eq!(tasks.len(), 2);
        // check both titles
        assert_eq!(tasks[0].title, "Buy milk");
        assert_eq!(tasks[1].title, "Learn Rust");
    }

    #[test]
    fn test_done_task() {
        // use setup_tasks()
        let mut tasks: Vec<Task> = setup_tasks();
        // run_command(Command::Done(1), &mut tasks)
        run_command(Command::Done(1), &mut tasks);
        // check tasks[0].done == true
        assert_eq!(tasks[0].done, true);
    }

    #[test]
    fn test_delete_task() {
        // use setup_tasks()
        let mut tasks: Vec<Task> = setup_tasks();
        // run_command(Command::Delete(1), &mut tasks)
        run_command(Command::Delete(1), &mut tasks);
        // check tasks.len() == 1
        assert_eq!(tasks.len(), 1);
        // check remaining task id is 2
        assert_eq!(tasks[0].id, 2);
    }

    #[test]
    fn test_invalid_command() {
        // parse_command("invalid")
        let result = parse_command("invalid");
        // check returns None
        assert!(result.is_none());
    }
    #[test]
    fn test_full_cli_lifecycle() {
        let mut tasks: Vec<Task> = Vec::new();

        // add 3 tasks using run_command

        let titles: Vec<&str> = vec!["Lern Rust", "Practice Rust", "Master Rust"];
        for title in titles {
            run_command(Command::Add(title.to_string()), &mut tasks);
        }
        // check count
        assert_eq!(tasks.len(), 3);
        // mark done
        run_command(Command::Done(1), &mut tasks);
        // check done status
        assert_eq!(tasks[0].done, true);
        // delete
        run_command(Command::Delete(1), &mut tasks);
        // check final state
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 2);
        assert_eq!(tasks[1].id, 3);
    }

    #[test]

    fn test_empty_task_Add() {
        let mut tasks: Vec<Task> = Vec::new();

        run_command(Command::Add(String::from("")), &mut tasks);

        assert_eq!(tasks.len(), 0)
    }

    #[test]

    fn test_done_nonexistent_task() {
        let mut tasks = setup_tasks();

        run_command(Command::Done(99), &mut tasks);

        assert!(tasks.iter().all(|t| !t.done))
    }
}
