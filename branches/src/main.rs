fn main() {
    let number = 3;

    if number < 5 {
        println!("Number is smaller than 5");
    } else {
        println!("Number is greater or equals 5");
    }

    // Using if for variable assignment
    let from_if = if number == 3 { "Yeet" } else { "Nop" };
    println!("From if filled {}", from_if);

    loops();

    owernship();
}

fn loops() {
    loop {
        println!("This loops forever without a break statement");
        break;
    }

    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter; // loop can return values with break
        }
    };
    println!("Loop result: {}", loop_result);

    // while loop
    let mut while_counter = 0;
    while while_counter < 10 {
        while_counter += 1;
    };
    println!("While counter {}", while_counter);

    // For loop
    let items = [1, 2, 3, 4];
    for item in items {
        println!("Item: {}", item);
    }

    // countdown
    // the range includes [1, 2, 3] 
    // that means the lower bound is included, the upper bound is not
    for count in (1..4).rev() {
        println!("Count {}", count);
    }
}


fn owernship() {
    let book = String::from("American Psycho");
    move_ownership(book);
    // book.len(); impossible, since ownership of book got transferred to hand_off function
    
    // to pass a variable into a function and use it later, use a reference
    let mut movie = String::from("Avatar");
    borrow_ownership(&movie);
    println!("Movie lenght {}", movie.len());

    mutable_borrow_ownership(&mut movie);
    println!("I got returned a changed movie: {}", movie);
}

fn move_ownership(book: String) {
    println!("This is the book with a new owner {} and cant be used by its parent anymore", book)
}

fn borrow_ownership(movie: &String) {
    println!("This is the movie reference {}. It can still be used by the parent function", movie);

    // It is not possible to change the borrowed values
    // movie.push_str("CHANGED");
}

fn mutable_borrow_ownership(movie: &mut String) {
    println!("We can now change a passed value");
    movie.push_str(" 2");
}
