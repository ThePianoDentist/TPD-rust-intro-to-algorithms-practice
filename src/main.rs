pub mod sort;
use sort::*;

fn main() {
    let mut input = [9.5, 7.4, 9.2, 2.3, 9.9, 4.2, 25.9, 13.2];
    let print = true;
    match print{
        true => insertion_sort_debug(&mut input),
        false => insertion_sort(&mut input)
    }
    println!("{:?}", input);
}
