fn main() {
    let x = 5;
    println!("{} multiplied by itself is {}", x, multiply(x));
}

fn multiply(number: i32) -> i32 {
    println!("The passed in parameter is {}", number);
    number * number // return an i32 value
}

