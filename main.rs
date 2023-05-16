use std::collections::VecDeque;

// Definition of a binary tree node
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// Implementation of the binary tree
impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, data: T) {
        if data < self.data {
            if let Some(left) = &mut self.left {
                left.insert(data);
            } else {
                self.left = Some(Box::new(Node::new(data)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(data);
            } else {
                self.right = Some(Box::new(Node::new(data)));
            }
        }
    }

    fn breadth_first_search(&self) {
        if self.is_empty() {
            println!("Binary tree is empty.");
            return;
        }

        let mut queue: VecDeque<&Node<T>> = VecDeque::new();
        queue.push_back(&self);

        while let Some(node) = queue.pop_front() {
            print!("{} ", node.data);

            if let Some(left) = &node.left {
                queue.push_back(&left);
            }

            if let Some(right) = &node.right {
                queue.push_back(&right);
            }
        }

        println!();
    }

    fn is_empty(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

// Example usage:
fn main() {
    let mut binary_tree = Node::new(4);
    binary_tree.insert(2);
    binary_tree.insert(6);
    binary_tree.insert(1);
    binary_tree.insert(3);
    binary_tree.insert(5);
    binary_tree.insert(7);

    binary_tree.breadth_first_search();
}
