#![warn(clippy::all)]
// Rohit: Illustrating Vectors in Rust

fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3, 4];

    for i in &vec {
        println!("{i}");
    }

    vec.push(5);
    println!("{vec:?}");

    let popped: Option<i32> = vec.pop();

    match popped {
        Some(value) => println!("Popped: {value}"),
        None => println!("Index out of bounds"),
    }

    println!("{vec:?}");

    // Creating a Vector with different types using Enum
    enum Thing {
        Apple(i32),
        Person(String),
        Exists(bool),
    }

    let mut things = vec![
        Thing::Apple(42),
        Thing::Person("Rohit".to_string()),
        Thing::Exists(true),
    ];

    things.push(Thing::Apple(56));

    for thing in &things {
        match thing {
            Thing::Apple(count) => println!("Apple count: {count}"),
            Thing::Person(name) => println!("Person: {name}"),
            Thing::Exists(status) => println!("Exists: {status}"),
        }
    }
}
