const AA: i32 = 44; // constant

fn main (){

    // Integer Types
    let a = 30; // i = signed integer
    let mut b : u32 = 35; // u = unsigned integer, mut = mutable

    let y = { // Initialize a variable based on the return value inside the block
        let x = 3;
        x + 1 // the variable y will have the value of 4
    };

/* Length	Signed	Unsigned
   8-bit	i8	    u8
   16-bit	i16	    u16
   32-bit	i32	    u32
   64-bit	i64	    u64
   128-bit	i128	u128
   arch	    isize	usize */ // isize and usize depends on the computer the program is running on (64 bits for 64-bit architecture and 32 bits for 32-bit architecture)

    // Floating-Point Types
    let c = 2.0; // default type = f64
    let d: f32 = 20.5;

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
}
