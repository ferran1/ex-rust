fn main() {

    // To instantiate a programmer:
    let pg1 = Programmer {
        email: String::from("ferran1004@gmail.com"),
        github: String::from("https://github.com/ferran1"),
        blog: String::from("https://dev.to/ferran1"),
        age: 21,
    };

    println!("Email: {}, Github: {}, Blog: {}", pg1.email, pg1.github, pg1.blog);

    let pg2 = Programmer {
        email: String::from("jason123@dev.com"),
        github: String::from("https://github.com/jason123"),
        blog: String::from("https://dev.to/jason123"),
        age: 40,
    };

    println!("pg1 same as pg2? {} ", pg1.is_same_as(&pg2)); //  pg1 doesn't have the same email as pg2 so it prints false
    println!("is pg1 older than pg2? {}", pg1.is_older(&pg2));
    println!("Does programmer 1 have a longer email address compared to programmer 2? {}", pg1.has_longer_email_address(&pg2));

    // You can call an associate function with :: instead of the method (.) syntax
    let pg3 = Programmer::new(String::from("test@gmail.com"),
                          String::from("https://github.com/test123"),
                          String::from("https://github.com/test123"), 40);

    // Only change the email of p3
    let pg3 = Programmer::new(String::from("test123@gmail.com"), pg3.github, pg3.blog, pg3.age);
    println!("New email address of pg3 is {}", pg3.email);

    // When you want to update the struct without calling the new function, you can use ..struct_name for the remaining values that should stay the same
    let pg3 = Programmer {
        email: String::from("test12345@gmail.com"),
        ..pg3
    };

    println!("pg3: {:#?}", pg3);

    // Instantiate a tuple struct
    let my_margin = Margin(50, 40);
    println!("{:#?}", my_margin);

    // Instantiate a Unit struct
    let _nil = Nil;
    println!("{:#?}", _nil);

}

// Structs are like classes in Java, it's a named type to which you can assign state (attributes/fields) and behavior (methods/functions)
#[derive(Debug)] // With this you can print the struct in a println! using {:#?}
pub struct Programmer {    // Pub makes this struct accessible from other modules
    pub email: String,     // Values are also accessible from other modules with the pub keyword
    pub github: String,
    pub blog: String,
    pub age: i8,
}

// Use the "impl" keyword to add methods to a struct
// Methods are different from functions in that they're defined within the context of a struct (or Enum or a trait object).
impl Programmer {

    fn is_same_as(&self, other: &Programmer) -> bool { // The first param of a method is always self, which represents the instance on which the method is being invoked
        self.email == other.email
    }

    fn is_older(&self, other: &Programmer) -> bool {
        self.age > other.age
    }

    // Associated function to create a new instance of Programmer,
    // This is an associated function because it doesn't have the self parameter, they don't have an instance of the struct to work with. They are "associated" with the struct.
    // Think of it as "static" in a language like Java
    fn new(email: String, github: String, blog: String, age: i8) -> Programmer {
        Programmer {
            email,
            github,
            blog,
            age,
        }
    }
}

impl Programmer { // It's possible to use different impl blocks for the same struct
    fn has_longer_email_address(&self, other: &Programmer) -> bool {
        self.email.len() > other.email.len()
    }
}

// Tuple struct
#[derive(Debug)]
struct Margin (i32, i32);

// A unit struct (Struct without any values)
#[derive(Debug)]
struct Nil;




