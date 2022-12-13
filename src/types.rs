use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::{Rc, Weak},
    usize,
};
// pub trait ItemType: Eq + Clone + Debug + Send + Sync {}
// impl<T> ItemType for T where T: Eq + Clone + Debug + Send + Sync {}

type RcNode<T> = Rc<Node<T>>;
type WeakRcNode<T> = Weak<Node<T>>;
#[derive(Debug)]
pub struct Tree<T> {
    pub(crate) root: Node<T>,
    // pub(crate) closed_sets: HashSet<T>,
}

/// `Node<T>` represents the single node in a tree.
#[derive(Debug)]
pub struct Node<T> {
    pub(crate) item: Option<T>,
    pub(crate) children: Vec<Node<T>>,
    // Use Weak reference here to prevent the reference cycle.
    // parent: RefCell<WeakRcNode<T>>,
    // The node's neighbor is the one with the same value that is "to the right"
    // of it in the tree.
    // neighbor_node: RefCell<RcNode<T>>,
}

// impl<T: ItemType> Node<T> {
//     pub fn new(item: Option<T>) -> Self {
//         Node {
//             item,
//             children: RefCell::new(Vec::new()),
//             parent: RefCell::new(Weak::new()),
//             neighbor_node: RefCell::new(Rc::new(Node::new(None))),
//         }
//     }
//     pub fn add_node(self, item: T) -> RcNode<T> {
//         let mut children = self.children.borrow_mut();
//         let mut node = Node::new(Some(item));
//         let node = Rc::new(node);
//         children.push(node.clone());
//         node
//     }
// }
// // impl<T: ItemType> Clone for Node<T> {
// //     fn clone(&self) -> Self {
// //         *self
// //     }
// // }

// impl<T: ItemType> Tree<T> {
//     pub fn from_hashmap(hashmap: &HashMap<String, T>) -> Self {
//         let mut tree = Tree {
//             root: Node::new(None),
//             closed_sets: HashSet::new(),
//         };
//         let mut root = &tree.root;
//         hashmap.iter().for_each(|(key, value)| {
//             let mut children = root.children.borrow_mut();
//             let mut node = Node::new(Some(value.clone()));
//             let node = Rc::new(node);
//             children.push(node.clone());
//         });
//         tree
//     }
//     pub fn insert_node(&mut self, node: RcNode<T>, item: T) {
//         let mut children = node.children.borrow_mut();
//         let mut node = Node::new(Some(item));
//         let node = Rc::new(node);
//         children.push(node.clone());
//     }
//     // pub fn print_tree(&self) {
//     //     let mut queue: Vec<Rc<Node<T>>> = Vec::new();
//     //     // queue.push(self.root.clone());
//     //     while !queue.is_empty() {
//     //         let node = queue.remove(0);
//     //         println!("{:?}", node.item);
//     //         let children = node.children.borrow();
//     //         children.iter().for_each(|x| queue.push(x.clone()));
//     //     }
//     // }
// }
