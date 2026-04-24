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

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

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


fn print_str(str:&String){
    println!("my name is :{}",str);
}
#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn add_task(tasks: &mut Vec<Task>, id: u32, title: String, done: bool) {
    let task = Task {
        id: id,
        title: title,
        done: done,
    };
    tasks.push(task)
}

fn list_tasks(tasks: &[Task]){
     for task in tasks {
        println!("task id: {} | task title: {} | task status: {}", [task.id](http://task.id/),task.title,task.done)
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    add_task(&mut tasks, 1, String::from("leran rust"), false);
    add_task(&mut tasks, 1, String::from("leran rust"), false);
    add_task(&mut tasks, 1, String::from("leran rust"), false);
    list_tasks(&tasks);
}

#[cfg(test)]
mod tests{
 use super::*;
    #[test]
    fn test_list_is_empty_initially() {
    let mut tasks : Vec<Task> = Vec::new();
        assert_eq!(tasks.is_empty(),true)
    }

    #[test]
    fn test_list_has_correct_count() {
    let mut tasks : Vec<Task> = Vec::new();
         add_task(&mut tasks, 1, String::from("learn rust"), false);
          add_task(&mut tasks, 2, String::from("learn js"), false);
          assert_eq!(tasks.len(),2)
    }

    #[test]
    fn test_task_fields_are_correct() {
    let mut tasks : Vec<Task> = Vec::new();
         add_task(&mut tasks, 1, String::from("learn rust"), false);
          assert_eq!(tasks[0].title,"learn rust")
    }
}
