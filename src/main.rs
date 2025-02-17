fn main() {
    let s = String::from("Hello"); // Allocates memory on the heap
    println!("message from heap: {}", s);

    let mut s = 1234.to_string(); // Note: rules regarding mutability still apply
    println!("message from heap: {}", s);

    // Strings are mutable
    s.push_str("4567");
    println!("My string number: {}", s);
}  