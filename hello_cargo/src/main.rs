macro_rules! rustth {
    ($msg:expr) => {
        println!("Message: {}", $msg);
    };
}

fn main() {
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 20;
    println!("The value of y is now: {}", y);
    println!("Hello, world!");
    rustth!("Hello from the macro!");
    tuple_function();
}

fn tuple_function() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);
    
} 

