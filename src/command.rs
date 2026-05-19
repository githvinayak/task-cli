pub enum Command {
    Add(String),
    List,
    Done(u32),
    Delete(u32),
}

pub fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.splitn(2, " ").collect();
    match parts[0] {
        "add" => {
            if parts.len() < 2 {
                return None;
            } // ✅
            let title = parts[1].to_string();
            return Some(Command::Add(title));
        }
        "list" => return Some(Command::List),
        "done" => {
            if parts.len() < 2 {
                return None;
            } // ✅
            let task_id: u32 = parts[1].parse().unwrap();
            return Some(Command::Done(task_id));
        }
        "delete" => {
            if parts.len() < 2 {
                return None;
            } // ✅
            let task_id: u32 = parts[1].parse().unwrap();
            return Some(Command::Delete(task_id));
        }
        _ => None,
    }
}
