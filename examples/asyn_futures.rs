use async_std;
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
use std::vec::Vec;
use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn main() {
    let now = time::Instant::now();
    let _future_one = do_something(1);

    let _future_two = async { do_something(2).await };

    let _future_three = async {
        let outcome_one = do_something(2);
        let outcome_two = do_something(3);
        let res = join!(outcome_one, outcome_two);
        res.0 + res.1
    };

    let async_outcome = async {
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);
        futures_vec.push(future_four);
        futures_vec.push(future_five);
        let handles = futures_vec
            .into_iter()
            .map(async_std::task::spawn)
            .collect::<Vec<_>>();
        let res = join_all(handles).await;
        res.into_iter().sum::<i8>()
    };

    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);

    let outcome = block_on(async_outcome);
    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", outcome);
}
