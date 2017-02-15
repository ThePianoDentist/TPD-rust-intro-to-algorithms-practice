
/*pub struct RedBlackTree{
    
}

pub struct RedBlackNode{
    red: bool,
    parent: RedBlackNode,
    left: RedBlackNode,
    right: RedBlackNode,
    value: i64
}*/

#[derive(Clone, Debug)]
pub struct Node{
    parent: Option<Box<Node>>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i64
}

pub struct BST{
    pub root: Option<Box<Node>>
}

pub trait TreeOperations{
    //fn tree_search(BST, i64) -> Node;
    fn tree_insert(&mut self, i64);
    //fn tree_delete(BST, i64) -> Node;
}

impl TreeOperations for BST{

    fn tree_insert(&mut self, new_val: i64){
        let mut node: Option<Box<Node>> = self.root.clone();
        
        while node.is_some(){
            let a = node.unwrap();
            if new_val < a.value{
                node = a.left;
            }
            else{
                node = a.right;
            }
        }

        let new_node: Node = Node{
            parent: None,
            left: None,
            right: None,
            value: new_val
        };
        
        if !node.is_some(){
            self.root = Some(Box::new(new_node))
        }
        else{
            let mut a = node.unwrap();
            if new_val < a.value{
                a.left = Some(Box::new(new_node))
            }
            else{ 
                a.right = Some(Box::new(new_node))
            }
        }
    }
}
