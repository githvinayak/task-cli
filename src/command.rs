pub enum SortOrder {
    SortById,
    SortByStatus,
}

pub enum Command {
    Add(String),
    List(SortOrder),
    Done(u32),
    Delete(u32),
    Clear,
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
        "list" => {
            if parts.len() < 2 {
                return None;
            } // ✅
            match parts[1] {
                "--by-status" => Some(Command::List(SortOrder::SortByStatus)),
                "--by-id" => Some(Command::List(SortOrder::SortById)),
                _ => Some(Command::List(SortOrder::SortById)),
            }
        }
        "done" => {
            if parts.len() < 2 {
                return None;
            } // ✅
            //  let task_id: u32 =
            match parts[1].parse::<u32>() {
                Ok(task_id) => Some(Command::Done(task_id)),
                Err(_) => {
                    println!("⚠️ invalid id: '{}' — must be a number", parts[1]);
                    None
                }
            }
        }
        "delete" => {
            if parts.len() < 2 {
                return None;
            } // ✅
            if parts.len() < 2 {
                return None;
            } // ✅
            //  let task_id: u32 =
            match parts[1].parse::<u32>() {
                Ok(task_id) => Some(Command::Delete(task_id)),
                Err(_) => {
                    println!("⚠️ invalid id: '{}' — must be a number", parts[1]);
                    None
                }
            }
        }
        "clear" => Some(Command::Clear),
        _ => None,
    }
}
