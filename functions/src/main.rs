fn main() {
    another_function(42);
    let returned_value = function_that_returns_a_value();
    println!("The function returned {}", returned_value);
}

fn another_function(parameter: i32) {
    println!("Another function called {}", parameter);

    let code_block_result = {
        let x = 1 + 2;
        // Does not have a ; at the end! 
        // A semicolon turns this into a statement and it does not return a value
        x 
    };
    println!("This is a code block result {}", code_block_result);
}

fn function_that_returns_a_value() -> i32 {
    return 42;
}
