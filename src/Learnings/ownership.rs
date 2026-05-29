// Rule 1 → Every value has exactly ONE owner
// Rule 2 → There can only be one owner at a time
// Rule 3 → When the owner goes out of scope, value is dropped

fn main(){
    // Rule 1 example:
    let a:i32  = 1;
    let b  = String::from("vini");
    println!("a is:{} and b is {}",a,b);
// Rule 2 example:
    let b = String::from("vini");
    let c = b;  // what happens to b now?
  println!("b is:{}",b);
    // Rule 3 example:
    {
       let c  = String::from("vini1");
       println!("c is {}", c); 
    }

}