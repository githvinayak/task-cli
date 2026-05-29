// without lifetime (Rust infers it here — works fine)
// fn find_task(tasks: &mut [Task], id: u32) -> Option<&mut Task>

// with explicit lifetime annotation
// fn find_task<'a>(tasks: &'a mut [Task], id: u32) -> Option<&'a mut Task>
//           ^^           ^^                                ^^
//     declare 'a    input has lifetime 'a         output has same lifetime 'a


struct Task {
    id:u32,
    title:String,
    done:bool
}
impl Task {
    fn new(id:u32,title:String)->Task{
        Task{
            id,
            title,
            done:false
        }
    }
}

fn longest_title<'a>(t1: &'a Task, t2: &'a Task) -> &'a str {
    if t1.title.len() > t2.title.len() {
        &t1.title
    } else {
        &t2.title
    }
}

fn main() {
    let t1 = Task::new(1, String::from("Buy milk"));
    let longest;

    
        let t2 = Task::new(2, String::from("Learn Rust today"));
        longest = longest_title(&t1, &t2);


    println!("{}", longest); // 💥 or ✅?
}