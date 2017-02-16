
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
pub struct Node{
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub value: i64
}

impl Node{
    pub fn insert(&mut self, new_val: i64){
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

    
    pub fn inorder(&mut self, func: &Fn(i64)){
        match self.left{
            Some(ref mut left) => left.inorder(func),
            None => {}
        };
        func(self.value);
        match self.right{
            Some(ref mut right) => right.inorder(func),
            None => {}
        };
    }

    pub fn preorder(&mut self, func: &Fn(i64)){
        func(self.value);
        match self.left{
            Some(ref mut left) => left.inorder(func),
            None => {}
        };
        match self.right{
            Some(ref mut right) => right.inorder(func),
            None => {}
        };
    }

    pub fn postorder(&mut self, func: &Fn(i64)){
        match self.left{
            Some(ref mut left) => left.inorder(func),
            None => {}
        };
        match self.right{
            Some(ref mut right) => right.inorder(func),
            None => {}
        };
        func(self.value);
    }

    pub fn print_inorder(&mut self){
        println!("In-order Traversal");
        let print = |x: i64| println!("{:?}", x);
        self.inorder(&print);
        println!("\n");
    }

    pub fn print_preorder(&mut self){
        println!("Pre-order Traversal");
        let print = |x: i64| println!("{:?}", x);
        self.preorder(&print);
        println!("\n");
    }

    pub fn print_postorder(&mut self){
        println!("Post-order Traversal");
        let print = |x: i64| println!("{:?}", x);
        self.postorder(&print);
        println!("\n");
    }
}
