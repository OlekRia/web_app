use core::fmt::Debug;

#[derive(Debug)]
struct TwoDposition {
    _x: i32,
    _y: i32,
}

#[derive(Debug)]
struct ThreeDposition {
    _x: i32,
    _y: i32,
    _z: i32,
}

fn print_debug<S>(s: &S)
where
    S: Debug,
{
    println!("{:?}", s);
}

fn _debug_iter<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for item in iter {
        println!("{:?}", item);
    }
}

fn main() {
    let two = TwoDposition { _x: 1, _y: 2 };
    let three = ThreeDposition {
        _x: 1,
        _y: 2,
        _z: 3,
    };
    print_debug(&two);
    print_debug(&three);
}
