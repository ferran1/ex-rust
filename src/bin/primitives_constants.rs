use std::mem;

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust"; // Static is a possible mutable variable with a static lifetime
// lifetime is the longest possible lifetime, and lasts for the lifetime of the running program

const THRESHOLD: i32 = 10; // Consts are always immutable

fn main (){

    /* Scalar types */

    // Integer Types
    let a = 30; // i = signed integer
    let mut b : u32 = 35; // u = unsigned integer, mut = mutable

    let y = { // Initialize a variable based on the return value inside the block
        let x = 3;
        x + 1 // The variable y will have the value of 4
    };

    println!("Variable y contains the number {}", y);

/* Length	Signed	Unsigned
   8-bit	i8	    u8
   16-bit	i16	    u16
   32-bit	i32	    u32
   64-bit	i64	    u64
   128-bit	i128	u128
   arch	    isize	usize */ // isize and usize depends on the computer the program is running on (64 bits for 64-bit architecture and 32 bits for 32-bit architecture)

    // Floating-Point Types
    let c = 2.0; // Default type = f64
    let d: f32 = 20.5; // Variables can always be type annotated

    // boolean
    let e = true;

    // char
    let f = 'F';

    // &str
    let my_str = "This is a &str";

    // String
    let my_string = String::from("This is a String");

    /* Calculations */
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    /* Compound types */

    // Arrays are collections of the same type with a fixed size
    let colors = ["Blue", "Green", "Yellow", "Purple", "Black"];
    println!("Blue color: {}", colors[0]); // Access elements by index

    let numbers = [5; 10]; // Will contain 10 elements of number 5

    for number in numbers.iter() {
        println!("{}", number);
    }

    println!("Numbers array size: {}", numbers.len()); // len() returns the size of the array
    println!("Array occupies {} bytes ", mem::size_of_val(&numbers)); // Arrays are stack allocated

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("Borrow a section of the array as a slice");
    analyze_slice(&numbers[1 .. 4]);

    // A tuple is a collection of different types
    let tup = (500, 6.4 , "test");
    let (x, y, z) = tup; // Deconstruct tuple to create bindings
    println!("The value of x is {}", x);

    // Tuples are printable
    println!("Tuple: {:?}", tup);

    // Access constants in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);

}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
