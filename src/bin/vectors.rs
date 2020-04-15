fn main(){

    // Vectors are collections of the same type that puts all the values next to each other in memory
    let mut v: Vec<i32> = Vec::new(); // Annotate the type when you don't want to insert values yet
    v.push(3); // Add elements to a vector using the push method (vector has to be mutable)
    v.push(7);
    let v2 = vec!["Blue", "Green", "Purple", "Yellow"]; // Create vector with type inference

    let third_value: &str = &v2[2]; // Get a specific value from the index
    println!("The third value: {}", third_value);

    match v.get(1) {
        Some(third_value) => println!("It's purple!"),
        None => println!("It's not purple!")
    }




} // Vectors are destroyed when out of scope
