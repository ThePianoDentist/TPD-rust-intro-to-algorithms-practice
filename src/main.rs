#![cfg_attr(feature="clippy", feature(plugin))]
pub mod sort;
use sort::*;
pub mod datastructures;
use datastructures::*;

fn main() {
    let mut input_f = [9.5, 7.4, 9.2, 2.3, 9.9, 4.2, 25.9, 13.2];
    let mut input_i = [4, 2, 9, 0, 5, 4, 1, 9];
    let print = false; 
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

    let mut bst = Node{left: None, right: None, value: 5};
    bst.insert(6);
    bst.insert(22);
    bst.insert(15);
    bst.insert(3);
    bst.insert(1);
    bst.insert(50);
    bst.print_inorder();
    bst.print_preorder();
    bst.print_postorder();
    bst.square();
    bst.print_inorder();
    let mut rb_bst = RedBlackNode{left: None, right: None, value: 10.0, colour: Colour::Red};
    rb_bst.insert(22.9);
    rb_bst.insert(13.1);
    rb_bst.insert(-12.2);
    //rb_bst.square();
    rb_bst.re_order();
    rb_bst.print_inorder();
    rb_bst.reciprocal();
    rb_bst.print_inorder();
    rb_bst.reverse();
    rb_bst.print_inorder();
    //bst.root.tree_insert(5).tree_insert(2).tree_insert(13).tree_insert(40);//.unwrap().print_in_order();
    //bst.root.tree_insert(2);
    //bst.root.tree_insert(13);
    //bst.root.tree_insert(40);
    //bst.root.unwrap().print_in_order();
}
