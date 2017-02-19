use std::fmt::Debug;
use std::cmp::PartialOrd;
use std::ops::Mul;
use std::ops::Div;
use std::marker::Copy;
use std::mem::swap;
pub extern crate num_traits;
use num_traits::One;

#[derive(Debug)]
pub struct Node<T>{
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: T
}

pub enum Colour{
    Red,
    Black
}

pub struct RedBlackNode<T>{
    pub colour: Colour,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: T
}

macro_rules! node_trait{
    ($U:ident) => (
    impl<T> $U<T> where T: Debug + Mul<Output = T> + Div<Output = T> + One + PartialOrd + Copy{
        pub fn insert(&mut self, new_val: T){
            let current_node = match new_val < self.value{
                true => &mut self.left,
                false => &mut self.right
            };
            match current_node{
                &mut Some(ref mut lower_node) => lower_node.insert(new_val),
                &mut None => {let new_node = Node{left: None, right: None, value: new_val};
                    let new_boxed = Some(Box::new(new_node));
                    *current_node = new_boxed;
                }
            };
        }

        pub fn inorder(&mut self, func: &Fn(&mut T)){
            match self.left{
                Some(ref mut left) => left.inorder(func),
                None => {}
            };
            func(&mut self.value);
            match self.right{
                Some(ref mut right) => right.inorder(func),
                None => {}
            };
        }

        pub fn preorder(&mut self, func: &Fn(&mut T)){
            func(&mut self.value);
            match self.left{
                Some(ref mut left) => left.inorder(func),
                None => {}
            };
            match self.right{
                Some(ref mut right) => right.inorder(func),
                None => {}
            };
        }

        pub fn postorder(&mut self, func: &Fn(&mut T)){
            match self.left{
                Some(ref mut left) => left.inorder(func),
                None => {}
            };
            match self.right{
                Some(ref mut right) => right.inorder(func),
                None => {}
            };
            func(&mut self.value);
        }

        pub fn print_inorder(&mut self){
            println!("In-order Traversal");
            let print = |x: &mut T| println!("{:?}", x);
            self.inorder(&print);
            println!("\n");
        }

        pub fn print_preorder(&mut self){
            println!("Pre-order Traversal");
            let print = |x: &mut T| println!("{:?}", x);
            self.preorder(&print);
            println!("\n");
        }

        pub fn print_postorder(&mut self){
            println!("Post-order Traversal");
            let print = |x: &mut T| println!("{:?}", x);
            self.postorder(&print);
            println!("\n");
        }

        pub fn square(&mut self){
            println!("Squaring tree");
            let square = |x: &mut T| *x = (*x) * (*x);
            self.preorder(&square);
        }

        pub fn reciprocal(&mut self){
            // check dividing by 0 breaking
            println!("Taking reciprocal of tree");
            let one = 1;
            let recip = |x: &mut T| *x = T::one() / (*x);
            self.preorder(&recip);
        }

        pub fn re_order(&mut self){
            //YEAH THIS ACTUAKLKY DOESNT WORK
            // does this actually work? need some more test cases
            //  also what is complexity of this?
            // while left bigger
            //    swap with left
            //    sort left
            // while right bigger
            //      swap with right
            //      sort right
            if let Some(ref mut left) = self.left{
                while left.value > self.value{    
                    swap(&mut left.value, &mut self.value);
                    left.re_order();
                }
            }
            if let Some(ref mut right) = self.right{
                while right.value < self.value{
                    swap(&mut right.value, &mut self.value);
                    right.re_order();
                }
            }
        }
    })
}

node_trait!(Node);
node_trait!(RedBlackNode);
