fn main() {

    let mut counter = 0;

    // Regular loop
    loop { // Keeps looping until the break keyword
        println!("The counter is {}", counter);

        counter += 1;

        if counter == 10 {
            break;
        }
    }

    // While loop
    while counter != 14 { // Keeps looping until condition is false
        println!("Counter {} is unequal to 14!", counter);

        counter += 2;
    }

    // For loop
    for number in (1..10).rev() { // Loop 1-10 range
        println!("{}", number);
    }

    let my_array = [15, 21, 1, 3, 99];

    for number in my_array.iter() { // Execute some code for each item in a collection
        println!("The value is {}", number);
    }
}
