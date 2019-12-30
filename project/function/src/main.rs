fn main() {
    println!("Hello, world!");
    another_function(5, 6);
    let x = plus_five(5);

    let y ={
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}",y);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32){
    println!("Another function.");
    println!("The value of x is: {}",x);
    println!("The value of y is: {}",y);
}

fn five() -> i32{
    5
}

fn plus_five(x: i32) -> i32{
    x+five()
}