fn main() {

    // Ownership
    let y = String::from("Hello");
    let a = y; /* Ownership is being "moved" to the variable a because the value
                       "hello" stored in the heap can't have 2 owners, this makes the old variable y invalid */

    // Will result in an error
    // println!("{}", y);

    // If you do want 2 variables with the same String you'll have to clone it
    let z = String::from("Hello");
    let b = z.clone();
    println!("both variable z: \"{}\" and variable c: \"{}\" are valid", z, b);

    let mut c = String::from("Hello world!");
    let d = &c;
    let e = &c; // variables d and e borrow from the variable c,
                        // while variable c is borrowed it'll be immutable

    let x = 33;
    print_value(&x); // With the & sign the print_value function will now "borrow" the variable x

    println!("Printing x: {}", x);

}

fn print_value(x: &i32) {
    println!("The passed in value is {}", x);
}
