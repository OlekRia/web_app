use std::{thread, thread::JoinHandle, time};

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn main() {
    let now = time::Instant::now();
    let one: JoinHandle<i8> = thread::spawn(|| do_something(1));
    let two: JoinHandle<i8> = thread::spawn(|| do_something(2));
    let three: JoinHandle<i8> = thread::spawn(|| do_something(3));

    let res1 = one.join();
    let res2 = two.join();
    let res3 = three.join();

    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", res1.unwrap() + res2.unwrap() + res3.unwrap());
}
