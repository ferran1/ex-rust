fn main () {

    let number = 1;
    let name = "Ferran";

    match number {
        1 => println!("It's number one!"),
        2 => println!("It's number two!"),
        1...5 => println!("It's one from 1 to 5"),
        _ => println!("It doesn't match")
    }

    match name {
        "Ferran" => println!("Hello Ferran!"),
        "James" => { // scope for match arm with multiple lines
            if number > 0 {
                println!("Hello James, the number is bigger than zero")
            }
        },
        _ => println!("Who are you?")
    }

    // If let (Shorter way of writing a match statement with one arm)
    if let number = 1 {
        println!("The number is 1!");
    } else {
        println!("The number isn't 1..")
    }

}
