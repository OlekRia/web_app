mod to_do;

use to_do::enums::TaskStatus::DONE;
use to_do::traits::{delete::Delete, edit::Edit, get::Get};
use to_do::{to_do_factory, ItemTypes};

// fn pr<T: std::fmt::Display>(display: T) {
//     println!("{}", display);
// }

fn main() {
    let to_do_item = to_do_factory("washing", DONE);

    match to_do_item {
        ItemTypes::Done(done) => {
            done.get(&done.super_struct.title);
            done.delete(&done.super_struct.title);
        }
        ItemTypes::Pending(pending) => {
            pending.get(&pending.super_struct.title);
            pending.set_to_done(&pending.super_struct.title);
        }
    }
}
