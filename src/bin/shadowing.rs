fn main () {

    let x = 15;

    {
        // Ignored because this binding only lives in this scope
        let x = x + 5;
    }

    // This binding "shadows" the previous binding using the let keyword
    let x = x + 1; // x = 16 now

    // Won't work because the variable x isn't mutable
    // x = x + 1;

    println!("{}", x);
}
