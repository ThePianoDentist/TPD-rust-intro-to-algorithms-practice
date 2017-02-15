#![cfg_attr(feature="clippy", feature(plugin))]
pub mod sort;
use sort::*;

fn main() {
    let mut input_f = [9.5, 7.4, 9.2, 2.3, 9.9, 4.2, 25.9, 13.2];
    let mut input_i = [4, 2, 9, 0, 5, 4, 1, 9];
    let print = true;
    match print{
        true => {
            insertion_sort_debug(&mut input_f);
            insertion_sort_debug(&mut input_i)
        },
        false => {
            insertion_sort(&mut input_f);
            insertion_sort(&mut input_i)
        }
    }
    println!("{:?}", input_f);
    println!("{:?}", input_i);
}
