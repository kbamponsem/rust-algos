use std::borrow::BorrowMut;

type Child = Option<Box<Node>>;

#[derive(Debug, PartialEq, Clone, Eq)]
struct Node {
    data: i32,
    left: Child,
    right: Child,
}

impl Node {
    pub fn new(data: i32) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
    pub fn has_children(&self) -> bool {
        self.left.is_some() || self.right.is_some()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Tree {
    root: Child,
}

impl Tree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, data: i32) -> &mut Self {
        if self.root.is_none() {
            // We directly insert the new element at the root
            *self.root.borrow_mut() = Some(Box::new(Node {
                data,
                left: None,
                right: None,
            }));
        } else {
            let mut node = &mut self.root;

            while let Some(inner_node) = node {
                /*
                   With binary-tree, it is a bit tricky,
                   you have to determine the path to follow with the incoming data

                */
                if data < inner_node.data {
                    if inner_node.left.is_none() {
                        *inner_node.left.borrow_mut() = Some(Box::new(Node::new(data)));
                        break;
                    }
                    node = &mut inner_node.left;
                } else if data > inner_node.data {
                    if inner_node.right.is_none() {
                        *inner_node.right.borrow_mut() = Some(Box::new(Node::new(data)));
                    }
                    node = &mut inner_node.right;
                } else {
                    break;
                }
            }
        }
        self
    }

    pub fn delete(&mut self, data: i32) -> &mut Self {
        // Check if its the only item;
        if let Some(root_node) = &self.root {
            if root_node.left.is_none() && root_node.right.is_none() {
                *self.root.borrow_mut() = None;
            }
        }

        if self.root.is_none() {
            println!("Empty tree");
        } else {
            let mut node = &mut self.root;

            while let Some(inner_node) = node {
                if data < inner_node.data {
                    if let Some(inner_left) = &inner_node.left {
                        if inner_left.left.is_none() && inner_left.right.is_none() {
                            *inner_node.left.borrow_mut() = None;
                        }
                    }
                    node = &mut inner_node.left;
                } else if data > inner_node.data {
                    // Pre-empt and set the right to none
                    if let Some(inner_right) = &inner_node.right {
                        if inner_right.right.is_none() && inner_right.left.is_none() {
                            *inner_node.right.borrow_mut() = None;
                        }
                    }
                    node = &mut inner_node.right;
                } else {
                    if let Some(inner_right) = &mut inner_node.right {
                        *inner_right.left.borrow_mut() = inner_node.left.clone();
                        *inner_node.left.borrow_mut() = None;
                        *inner_node.borrow_mut() = inner_right.clone();
                    } else if let Some(inner_left) = &mut inner_node.left {
                        *inner_left.right.borrow_mut() = inner_node.right.clone();
                        *inner_node.right.borrow_mut() = None;
                        *inner_node.borrow_mut() = inner_left.clone();
                    }

                    break;
                }
            }
        }

        self
    }

    pub fn max_depth(&self) -> usize {
        // The idea is to recursively traverse the tree
        // and keep track of the depth

        let node = &self.root;
        // let mut leftcount =
        if let Some(inner_node) = node {
            if inner_node.left.is_none() && inner_node.right.is_none() {
                return 1;
            }

            let left_count = Tree {
                root: inner_node.left.clone(),
            }
            .max_depth();
            let right_count = Tree {
                root: inner_node.right.clone(),
            }
            .max_depth();
            let score = left_count + right_count;
            println!("{inner_node:?} -> {score:?}");
            return score;
        } else {
            1
        }
    }

    pub fn flip(&mut self) -> &mut Self {
        if let Some(inner_node) = &mut self.root {
            // Go through the tree, each node at a time,
            // Exchange left and right and then go to the left, then the right and do the same thing.
            // Then set up a temp to store inner_left
            let mut tree = Tree {
                root: inner_node.left.clone(),
            };
            let temp = tree.flip();
            let mut tree = Tree {
                root: inner_node.right.clone(),
            };
            *inner_node.left.borrow_mut() = tree.flip().root.clone();

            *inner_node.right.borrow_mut() = temp.root.clone();
        }

        self
    }
}

fn main() {
    println!("Hello, world!");

    let mut tree = Tree::new();

    tree.insert(4).insert(5).insert(-1).insert(2);
    tree.delete(5);
    // println!("{:#?}", tree);
}

#[cfg(test)]
mod tests {
    use crate::{Node, Tree};
    #[test]
    fn test_insertion() {
        let mut tree = Tree::new();

        tree.insert(4).insert(5).insert(-1).insert(2).insert(3);
        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 4,
                    left: Some(Box::new(Node {
                        data: -1,
                        left: None,
                        right: Some(Box::new(Node {
                            data: 2,
                            left: None,
                            right: Some(Box::new(Node::new(3)))
                        }))
                    })),
                    right: Some(Box::new(Node {
                        data: 5,
                        left: None,
                        right: None
                    }))
                }))
            }
        )
    }

    #[test]
    fn test_deletion() {
        let mut tree = Tree::new();

        tree.insert(4).insert(5).insert(-1).insert(2).insert(3);
        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 4,
                    left: Some(Box::new(Node {
                        data: -1,
                        left: None,
                        right: Some(Box::new(Node {
                            data: 2,
                            left: None,
                            right: Some(Box::new(Node::new(3)))
                        }))
                    })),
                    right: Some(Box::new(Node {
                        data: 5,
                        left: None,
                        right: None
                    }))
                }))
            }
        );

        tree.delete(5);
        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 4,
                    left: Some(Box::new(Node {
                        data: -1,
                        left: None,
                        right: Some(Box::new(Node {
                            data: 2,
                            left: None,
                            right: Some(Box::new(Node::new(3)))
                        }))
                    })),
                    right: None
                }))
            }
        );

        tree.delete(-1);
        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 4,
                    left: Some(Box::new(Node {
                        data: 2,
                        left: None,
                        right: Some(Box::new(Node::new(3)))
                    })),
                    right: None
                }))
            }
        );

        tree.delete(-2);
        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 4,
                    left: Some(Box::new(Node {
                        data: 2,
                        left: None,
                        right: Some(Box::new(Node::new(3)))
                    })),
                    right: None
                }))
            }
        );
    }

    #[test]
    fn simple_tree() {
        let mut tree = Tree::new();

        tree.insert(3).insert(2).insert(4);

        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 3,
                    left: Some(Box::new(Node::new(2))),
                    right: Some(Box::new(Node::new(4)))
                }))
            }
        );

        tree.delete(3);

        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 4,
                    left: Some(Box::new(Node::new(2))),
                    right: None
                }))
            }
        );

        tree.delete(2);
        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 4,
                    left: None,
                    right: None
                }))
            }
        );

        tree.delete(4);
        assert_eq!(tree, Tree { root: None });
    }

    #[test]
    fn test_max_depth() {
        let tree = Tree {
            root: Some(Box::new(Node {
                data: 3,
                left: Some(Box::new(Node::new(9))),
                right: Some(Box::new(Node {
                    data: 20,
                    left: Some(Box::new(Node::new(15))),
                    right: Some(Box::new(Node::new(7))),
                })),
            })),
        };

        assert_eq!(tree.max_depth(), 3);

        assert_eq!(
            Tree {
                root: Some(Box::new(Node {
                    data: 1,
                    left: None,
                    right: Some(Box::new(Node::new(2)))
                }))
            }
            .max_depth(),
            2
        );
    }

    #[test]
    fn test_flip() {
        let mut tree = Tree {
            root: Some(Box::new(Node {
                data: 3,
                left: Some(Box::new(Node::new(9))),
                right: Some(Box::new(Node {
                    data: 20,
                    left: Some(Box::new(Node::new(15))),
                    right: Some(Box::new(Node::new(7))),
                })),
            })),
        };

        tree.flip();

        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 3,
                    left: Some(Box::new(Node {
                        data: 20,
                        left: Some(Box::new(Node::new(7))),
                        right: Some(Box::new(Node::new(15))),
                    })),
                    right: Some(Box::new(Node::new(9)))
                }))
            }
        );

        let mut tree = Tree {
            root: Some(Box::new(Node {
                data: 2,
                left: Some(Box::new(Node::new(1))),
                right: Some(Box::new(Node::new(3))),
            })),
        };

        tree.flip();

        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 2,
                    left: Some(Box::new(Node::new(3))),
                    right: Some(Box::new(Node::new(1))),
                }))
            }
        );

        let mut tree = Tree {
            root: Some(Box::new(Node {
                data: 4,
                left: Some(Box::new(Node {
                    data: 2,
                    left: Some(Box::new(Node::new(1))),
                    right: Some(Box::new(Node::new(3))),
                })),
                right: Some(Box::new(Node {
                    data: 7,
                    left: Some(Box::new(Node::new(6))),
                    right: Some(Box::new(Node::new(9))),
                })),
            })),
        };

        tree.flip();
        assert_eq!(
            tree,
            Tree {
                root: Some(Box::new(Node {
                    data: 4,
                    left: Some(Box::new(Node {
                        data: 7,
                        left: Some(Box::new(Node::new(9))),
                        right: Some(Box::new(Node::new(6))),
                    })),
                    right: Some(Box::new(Node {
                        data: 2,
                        left: Some(Box::new(Node::new(3))),
                        right: Some(Box::new(Node::new(1))),
                    }))
                })),
            }
        );
    }
}
