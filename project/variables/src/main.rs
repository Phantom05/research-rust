fn main() {
    const MAX_POINTS : u32 = 100_000;
    let mut x = 5;
    println!("The value of x is : {}",x);
    x = 6;
    println!("The value of x is : {}",x);
    println!("{}",MAX_POINTS);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}",spaces);


    // let guess: u32 = "42".parse().expect("Not a number!");

    // let x = 2.0;
    // let y: f32 = 3.0;

    // // addition
    // let sum = 5+10;
    // // subtraction
    // let differnence =95.5 - 4.3;
    // // division
    // let quotient = 56.7 / 32.2;
    // // remainder
    // let remainder = 43 % 5;

    // println!("!! {}",quotient);


    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // let c = 'z';
    // let heart_eyed_cat =  '😻';

    // let tup: (i32,f64, u8) = (500, 6.4, 1);

    // let (x,y,z) = tup;
    // println!("The value of y is: {}", y);


    let x: (i32,f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{}",five_hundred);
    println!("{}",six_point_four);
    println!("{}",one);

    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let first = months[10];
    println!("The value of element is: {}",first);
}
