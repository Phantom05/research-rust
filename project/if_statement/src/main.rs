fn main() {
    let x = plus_five(5);
    println!("Hello, If!");
    println!("Hello, x: {}",x);
    
}

fn five() -> i32{
    5
}

fn plus_five(x:i32) -> i32{
    x + five()
}