fn add_doubles(closure: Box<dyn Fn(i32) -> i32>, one: i32, two: i32) -> i32 {
    closure(one) + closure(two)
}

fn main() {
    let closure = |int_input| int_input * 2;

    let outcome = add_doubles(Box::new(closure), 2, 3);

    println!("{}", outcome);
}

// Basic::

// fn main() {
//     let another_string = "case";
//     let test_closure = move |x| {
//         println!("{} {}", x, another_string);
//     };

//     test_closure("test");
//     println!("{}", another_string);
// }
