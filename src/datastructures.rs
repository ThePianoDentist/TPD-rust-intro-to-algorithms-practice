use std::fmt::Debug;
use std::cmp::PartialOrd;
use std::ops::Mul;
use std::ops::Div;
use std::marker::Copy;
use std::mem::swap;
pub extern crate num_traits;
use num_traits::One;
use num_traits::Zero;

#[derive(Debug)]
pub struct Node<T>{
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: T
}

#[derive(PartialEq)]
pub enum Colour{
    Red,
    Black
}

pub struct RedBlackNode<T>{
    pub colour: Colour,
    pub left: Option<Box<RedBlackNode<T>>>,
    pub right: Option<Box<RedBlackNode<T>>>,
    pub value: T
}

trait Constructor<T>{
    fn new(new_val: T) -> Self;
}

impl<T> Constructor<T> for Node<T>{
    fn new(new_val: T) -> Node<T>{
        Node{value: new_val, left: None, right: None}
    }
}

impl<T> Constructor<T> for RedBlackNode<T>{
    fn new(new_val: T) -> RedBlackNode<T>{
        RedBlackNode{value: new_val, left: None, right: None, colour: Colour::Red}
    }
}

macro_rules! node_trait{
    ($U:ident) => (
    impl<T> $U<T> where T: Debug + Mul<Output = T> + Div<Output = T> + One + Zero + PartialOrd + Copy{
        pub fn insert(&mut self, new_val: T){
            let current_node = match new_val < self.value{
                true => &mut self.left,
                false => &mut self.right
            };
            match current_node{
                &mut Some(ref mut lower_node) => lower_node.insert(new_val),
                &mut None => {
                    let new_node = $U::new(new_val);
                    let new_boxed = Some(Box::new(new_node));
                    *current_node = new_boxed;
                }
            };
        }
        
        // This removes both of a duplicate
        pub fn remove(&mut self, value: T){
            match self.left{
                Some(ref mut left) => {
                    if left.value == value{
                        left = match (left.left, left.right){
                            (Some(ref mut l_left), Some(ref mut l_right)) => {l_left},
                            (Some(ref mut l_left), None) => l_left,
                            (None, Some(ref mut l_right)) => l_right,
                            (None, None) => None
                        };
                    }
                    else{
                        left.remove(value);
                    }
                },
                None => {}
            };
            /*
            match self.right{
                Some(ref right) => { 
                    if right.value == value{
                        *right = match (right.left, right.right){
                            (Some(ref r_left), Some(ref r_right)) => {r_left},
                            (Some(ref r_left), None) => r_left,
                            (None, Some(ref r_right)) => r_right,
                            (None, None) => None
                        };
                    }
                    else{
                        right.remove(value);
                    }
                },
                None => {}
            };*/
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

        pub fn search(&self, search_value: T) -> Option<&$U<T>>{
            // TODO Im ignoring my preorder func. can I refactor?
            if self.value == search_value{
                println!("Found value: {:?}", search_value);
                return Some(self)
            }
            match self.left{
                Some(ref left) => left.search(search_value),
                None => {None}
            };
            match self.right{
                Some(ref right) => right.search(search_value),
                None => {None}
            };
            return None
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
            let recip = |x: &mut T| *x = T::one() / (*x);
            self.preorder(&recip);
        }

        pub fn reverse(&mut self){
            swap(&mut self.left, &mut self.right);
            if let Some(ref mut left) = self.left{
                left.reverse();
            }
            if let Some(ref mut right) = self.right{
                right.reverse();
            }
        }
        /*
        // Reverse only if same sign
        // Re-orders bst in case of reciprocal of mixed sign tree
        pub fn smart_reverse(&mut self){
            let mut do_swap = true;
            if let Some(ref left) = self.left{
                if let Some(ref right) = self.right{
                    if left.value * right.value < T::zero(){
                        do_swap = false;
                    }
                }
            }
            if do_swap{
                swap(&mut self.left, &mut self.right);
            }
            if let Some(ref mut left) = self.left{
                left.smart_reverse();
            }
            if let Some(ref mut right) = self.right{
                right.smart_reverse();
            }
        }
        */
        // Reverse only if same sign
        // Re-orders bst in case of reciprocal of mixed sign tree
        // no it does not lol!!! Think it needs balanced tree for reverse to re-order correctly?
        pub fn smart_reverse(&mut self, parent_val: T){
            let mut do_swap = true;
            if let Some(ref left) = self.left{
                if let Some(ref right) = self.right{
                    if left.value * right.value < T::zero(){
                        do_swap = false;
                    }
                }
                else{
                    if parent_val * left.value < T::zero(){
                        do_swap = false;
                    }
                }
            }
            else if let Some(ref right) = self.right{
                if parent_val * right.value < T::zero(){
                    do_swap = false;
                }
            }
            if do_swap{
                swap(&mut self.left, &mut self.right);
            }
            if let Some(ref mut left) = self.left{
                left.smart_reverse(self.value);
            }
            if let Some(ref mut right) = self.right{
                right.smart_reverse(self.value);
            }
        }
        

        pub fn re_order(&mut self){
            println!("Not implemented!");
            return
            // leaving this for now as most functions applied to whole tree would require
            // blind reversing. simpler than full re-order.
            // http://stackoverflow.com/questions/2577098/how-to-convert-a-binary-tree-to-binary-search-tree-in-place-i-e-we-cannot-use
            // for what im trying to do
            //YEAH THIS ACTUAKLKY DOESNT WORK
            // does this actually work? need some more test cases
            //  also what is complexity of this?
            println!("Value: {:?}", self.value);
            if let Some(ref mut left) = self.left{
                println!("Left value: {:?}", left.value);
                while left.value > self.value{    
                    println!("Swap left");
                    swap(&mut left.value, &mut self.value);
                    left.re_order();
                }
            }
            else{println!("No left");}
            if let Some(ref mut right) = self.right{
                println!("Right value: {:?}", right.value);
                while right.value < self.value{
                    println!("Swap right...{:?}", self.value);
                    swap(&mut right.value, &mut self.value);
                    println!("to ... {:?}", self.value);
                    right.re_order();
                }
            }
            else{println!("No right");}
        }
    })
}

node_trait!(Node);
node_trait!(RedBlackNode);

pub trait IsValid{
    fn is_valid(&self) -> bool;
}

impl<T> IsValid for Node<T> where T: Debug + PartialOrd{
    fn is_valid(&self) -> bool {
        if let Some(ref left) = self.left{
            if left.value > self.value{
                return false;
            }
            else{
                return left.is_valid();
            }
        }
        else if let Some(ref right) = self.right{
            if right.value < self.value{
                return false;
            }
            else{
                return right.is_valid();
            }
        }
        else{
            return true;
        }
        
    }
}

impl<T> IsValid for RedBlackNode<T> where T: Debug + PartialOrd{
    fn is_valid(&self) -> bool {
        if let Some(ref left) = self.left{
            if left.value > self.value || left.colour == self.colour{
                return false;
            }
            else{
                return left.is_valid();
            }
        }
        else if let Some(ref right) = self.right{
            if right.value < self.value || right.colour == self.colour{
                return false;
            }
            else{
                return right.is_valid();
            }
        }
        else{
            return true;
        }
        
    }
}
