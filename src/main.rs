
struct Task {
    id: i32,
    title: String,
    done: bool,
}

fn main() {
    // integers
    let task1 = Task {
        id: 1,
        title: String::from("Learn Rust"),
        done: false,
    };
    let x: i32 = 5;
    let y = x;
    println!("x:{} y:{}", x, y);

    // floats
    let a: f64 = 3.14;
    let b = a;
    println!("a:{} b:{}", a, b);

    // bool
    let t: bool = true;
    let f = t;
    println!("t:{} f:{}", t, f);

    // char
    let c1: char = 'R';
    let c2 = c1;
    println!("c1:{} c2:{}", c1, c2);

    // now String
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1:{}", s1); // what happens here?
    println!(
        "task id:{} and task title:{} and task status:{}",
        task1.id, task1.title, task1.done
    )
}
