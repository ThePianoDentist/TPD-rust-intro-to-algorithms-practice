pub mod sort;
use sort::*;
fn main() {
    let mut input = [9.5, 7.4, 9.2, 2.3, 9.9, 4.2, 25.9, 13.2];
    debug_insertion_sort(&mut input);
    println!("{:?}", input);
}
