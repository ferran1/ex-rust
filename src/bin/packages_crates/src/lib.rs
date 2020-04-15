// Shows how to use packages, creates, relative paths and absolute paths

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::example_function();
    }
}

mod a_module {
    pub mod another_module {
        pub fn do_something(){
        }
    }
}

fn example_function() {
    // Call do_something by the absolute path
    crate::a_module::another_module::do_something();

}
