use crate::task::Task;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    // 1. check if file exists
    if std::path::Path::new(FILE_PATH).exists(){
    
     let content = match std::fs::read_to_string(FILE_PATH){
        Ok(c)=> c,
        Err(e)=>{
         println!("⚠️ Could not read file: {}", e);
           return Vec::new();
        }
     };

     match serde_json::from_str(&content){
        Ok(tasks)=> tasks,
        Err(e)=>{
         println!("⚠️ Could not read file: {}", e);
          return Vec::new();
        }
     }
    }else{
         Vec::new()
    }
}

pub fn save_tasks(tasks: &[Task]) {
    // 1. convert Vec<Task> to JSON string
    // let content = 
   let content = match serde_json::to_string_pretty(tasks){
        Ok(c)=> c,
        Err(e)=>{
            println!("⚠️ could not serialize tasks: {}", e);
            return;
        }
    };
    // 2. wlrite to tasks.json
    match std::fs::write(FILE_PATH, content){
        Ok(_)=> {},
        Err(e)=>{
            println!("⚠️ could not save tasks: {}", e);
        }
    };
}