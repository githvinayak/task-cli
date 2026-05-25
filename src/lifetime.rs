// without lifetime (Rust infers it here — works fine)
fn find_task(tasks: &mut [Task], id: u32) -> Option<&mut Task>

// with explicit lifetime annotation
fn find_task<'a>(tasks: &'a mut [Task], id: u32) -> Option<&'a mut Task>
//           ^^           ^^                                ^^
//     declare 'a    input has lifetime 'a         output has same lifetime 'a