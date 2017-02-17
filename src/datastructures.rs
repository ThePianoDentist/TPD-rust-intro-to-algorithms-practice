use std::fmt::Debug;
use std::cmp::PartialOrd;
use std::ops::Mul;
use std::marker::Copy;
/*pub struct RedBlackTree{
    
}

pub struct RedBlackNode{
    red: bool,
    parent: RedBlackNode,
    left: RedBlackNode,
    right: RedBlackNode,
    value: i64
}*/

#[derive(Debug)]
pub struct Node<T>{
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: T
}

impl<T> Node<T> where T: Debug + Mul + PartialOrd + Clone{
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
        let square = |x: &mut T| x = &mut ((*x) * (*x));
        self.preorder(&square);
    }
}
