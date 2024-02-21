use core::fmt::Debug;

#[derive(Debug)]
struct TwoDposition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ThreeDposition {
    x: i32,
    y: i32,
    z: i32,
}

fn print_debug<S>(s: &S)
where
    S: Debug,
{
    println!("{:?}", s);
}

fn debug_iter<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for item in iter {
        println!("{:?}", item);
    }
}

fn main() {
    let two = TwoDposition { x: 1, y: 2 };
    let three = ThreeDposition { x: 1, y: 2, z: 3 };
    print_debug(&two);
    print_debug(&three);
}
