fn main() {
    // Immutability
    let _a = 10;

    // Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let shadowed = 5;
    {
        let shadowed = 10;
        println!("The inner scope shadow value: {shadowed}");
    }
    println!("The outer scope original value: {shadowed}");

    // Shadowing with different type
    let _spaces = "   ";
    let _spaces = _spaces.len();

    // Type Annotation
    let m_god: i32 = "42".parse().expect("Parse failed");
    println!("Musiala Nummer: {}", m_god);

    // Tuples
    let tuple: (i32, u32, f32) = (100, 200, 420.69);
    let (_, _, nice) = tuple;
    println!("Yo this is the second {} and this the last {} value from a tuple", tuple.0, nice);

    // Arrays
    let _array = [1, 2, 3];
    let _array_with_type: [i32; 3] = [1, 2, 3]; // 3 elements of type i32
    let _array_with_same_values = [10; 3]; // 3 elements with value 10
}
