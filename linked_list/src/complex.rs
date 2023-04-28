use std::borrow::{Borrow, BorrowMut};

use crate::List;

#[derive(Debug, Default, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd)]
pub struct LinkedList {
    pub head: Option<Box<Node>>,
}

impl List for LinkedList {
    fn insert(&mut self, value: i32) -> &mut dyn List {
        self.insert(value);
        self
    }

    fn delete(&mut self, value: i32) -> &mut dyn List {
        self.delete(value);
        self
    }
}
impl LinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert(&mut self, data: i32) -> &mut Self {
        if self.head.is_none() {
            *self.head.borrow_mut() = Some(Box::new(Node { data, next: None }));
        } else {
            let mut node = &mut self.head;

            while let Some(inner_node) = node {
                // Trick:
                // Check if inner_node.next has an element, if not, insert new data into inner_node.next.
                // If has element:
                //  set node to inner_node.next

                if inner_node.next.is_none() {
                    // Borrow inner_node.next as mutable and insert value
                    *inner_node.next.borrow_mut() = Some(Box::new(Node { data, next: None }));
                    break;
                } else {
                    node = &mut inner_node.next;
                }
            }
        }
        self
    }
    pub fn len(&self) -> usize {
        if self.head.is_none() {
            0
        } else {
            let mut size = 0;
            let mut node = &self.head;

            while let Some(inner_node) = node {
                size += 1;
                node = &inner_node.next;
            }

            size
        }
    }

    pub fn reverse_k_group(mut list: Option<Box<Node>>, k: usize) -> Option<Box<Node>> {
        let big_size = LinkedList { head: list.clone() }.len();
        let mut node = &mut list;
        let mut temp_inner_next = None;
        let mut temp_next = None;

        while let Some(inner_node) = node {
            /*
                At the beginning of the big list, we will compute the length
            */

            if let Some(inner_next) = &mut inner_node.next {
                println!("{:?} {:?}", inner_node.data, inner_next.data);
                temp_inner_next = inner_next.next.clone();
                *inner_next.next.borrow_mut() = None;
            }

            temp_next = inner_node.next.clone();
            *inner_node.next.borrow_mut() = None;

            // Swap inner_node and temp_next;
            *inner_node.next.borrow_mut() = temp_inner_next.clone();
            if let Some(inner_next) = &mut temp_next {
                *inner_next.next.borrow_mut() = Some(inner_node.clone());

                println!("Node: {inner_next:?} ");
            }
            break;
            // node = &mut inner_next.next;
        }
        todo!()
    }

    pub fn delete(&mut self, data: i32) -> Option<i32> {
        // if list is empty, i.e., head is none, no-op and return self;

        // Check case where the first item has the element

        if let Some(head_value) = &mut self.head {
            if head_value.data == data {
                // This means it's the element to the head
                *self.head.borrow_mut() = head_value.next.clone();
                return Some(data);
            }
        }

        let mut node = &mut self.head;

        while let Some(inner_node) = node {
            // Use pre-emption to detect if the next item has the value.
            // if next item has value:
            //  set current node.next to node.next.next
            if let Some(next_node) = &inner_node.next {
                if next_node.data == data {
                    *inner_node.next.borrow_mut() = next_node.next.clone();
                    return Some(data);
                }
            }
            node = &mut inner_node.next;
        }
        None
    }

    pub fn delete_nth_from_end(&mut self, n: usize) -> &mut Self {
        let index: usize = self.len() - (n);
        let mut counter = 0;

        // Edge case:
        // Only one element:
        if let Some(head_object) = &self.head {
            if head_object.next.is_none() && n == 1 {
                *self.head.borrow_mut() = None;
                return self;
            }
        }
        let mut node = &mut self.head;

        while let Some(inner_node) = node {
            println!("Counter: {}", counter);

            if counter == index - 1 {
                if let Some(child) = &inner_node.next {
                    *inner_node.next.borrow_mut() = child.next.clone();
                    break;
                }
            }
            counter += 1;
            node = &mut inner_node.next;
        }

        self
    }

    pub fn peek(&self) -> Option<i32> {
        if let Some(node) = &self.head {
            return Some(node.data);
        }
        None
    }
    pub fn pop_front(&mut self) -> Option<i32> {
        // Pop front is the same us deleting the element at the front.
        // We can get away with this by reading the value at head and deleting it
        if let Some(value) = self.peek() {
            return self.delete(value);
        }
        None
    }

    fn merge(mut left: LinkedList, mut right: LinkedList) -> LinkedList {
        let mut out_list = LinkedList::new();
        // We will traverse the left and right until one of them is done.
        while left.head.is_some() && right.head.is_some() {
            // just like the merge in merge_sort,
            // We will peek on both left and right, to compare the values,
            // if the left is smaller, we insert that into the out_list
            // else: we insert right's value
            let left_val = left.peek().unwrap();
            let right_val = right.peek().unwrap();
            if left_val <= right_val {
                out_list.insert(left.pop_front().unwrap());
            } else {
                out_list.insert(right.pop_front().unwrap());
            }
        }

        // At this point, there could be still elements in the either lists,
        // so we make sure we keep popping until there are not elements in
        // either left or right
        while left.head.is_some() {
            out_list.insert(left.pop_front().unwrap());
        }
        while right.head.is_some() {
            out_list.insert(right.pop_front().unwrap());
        }
        out_list
    }
    pub fn merge_list(mut list: Vec<LinkedList>) -> LinkedList {
        let out = LinkedList::new();

        list.iter()
            .fold(out.clone(), |left, right| Self::merge(left, right.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::{LinkedList, Node};

    #[test]
    // #[ignore = "Not yet ready"]

    fn test_insertion() {
        let mut list = LinkedList::new();

        list.insert(1).insert(2).insert(3);

        assert_eq!(
            list,
            LinkedList {
                head: Some(Box::new(Node {
                    data: 1,
                    next: Some(Box::new(Node {
                        data: 2,
                        next: Some(Box::new(Node {
                            data: 3,
                            next: None
                        }))
                    }))
                }))
            }
        );
    }

    #[test]
    // #[ignore = "Not yet ready"]
    fn test_deletion() {
        let mut list = LinkedList::new();

        list.insert(1).insert(2).insert(3);

        list.delete(2);
        assert_eq!(
            list,
            LinkedList {
                head: Some(Box::new(Node {
                    data: 1,
                    next: Some(Box::new(Node {
                        data: 3,
                        next: None
                    }))
                }))
            }
        );

        list.delete(1);
        assert_eq!(
            list,
            LinkedList {
                head: Some(Box::new(Node {
                    data: 3,
                    next: None
                }))
            }
        );

        list.delete(4);
        assert_eq!(
            list,
            LinkedList {
                head: Some(Box::new(Node {
                    data: 3,
                    next: None
                }))
            }
        );

        list.delete(3);
        assert_eq!(list, LinkedList { head: None });
    }
    #[test]
    // #[ignore = "Not yet ready"]
    fn test_merge_list() {
        let lists = vec![
            LinkedList {
                head: Some(Box::new(Node {
                    data: 1,
                    next: Some(Box::new(Node {
                        data: 2,
                        next: Some(Box::new(Node {
                            data: 3,
                            next: None,
                        })),
                    })),
                })),
            },
            LinkedList {
                head: Some(Box::new(Node {
                    data: 3,
                    next: Some(Box::new(Node {
                        data: 6,
                        next: Some(Box::new(Node {
                            data: 7,
                            next: None,
                        })),
                    })),
                })),
            },
            LinkedList {
                head: Some(Box::new(Node {
                    data: 1,
                    next: Some(Box::new(Node {
                        data: 1,
                        next: None,
                    })),
                })),
            },
        ];

        assert_eq!(
            LinkedList::merge_list(lists),
            LinkedList {
                head: Some(Box::new(Node {
                    data: 1,
                    next: Some(Box::new(Node {
                        data: 1,
                        next: Some(Box::new(Node {
                            data: 1,
                            next: Some(Box::new(Node {
                                data: 2,
                                next: Some(Box::new(Node {
                                    data: 3,
                                    next: Some(Box::new(Node {
                                        data: 3,
                                        next: Some(Box::new(Node {
                                            data: 6,
                                            next: Some(Box::new(Node {
                                                data: 7,
                                                next: None
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }
        );
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::new();

        list.insert(1).insert(2).insert(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    #[ignore = "Not yet ready"]
    fn test_reverse_k_group() {
        let mut list = LinkedList::new();

        list.insert(1).insert(2).insert(3).insert(4).insert(5);

        assert_eq!(
            LinkedList::reverse_k_group(list.head, 2),
            Some(Box::new(Node {
                data: 2,
                next: Some(Box::new(Node {
                    data: 1,
                    next: Some(Box::new(Node {
                        data: 4,
                        next: Some(Box::new(Node {
                            data: 3,
                            next: Some(Box::new(Node {
                                data: 5,
                                next: None
                            }))
                        }))
                    }))
                }))
            }))
        );

        let mut list = LinkedList::new();
        list.insert(1).insert(2).insert(3).insert(4).insert(5);

        assert_eq!(
            LinkedList::reverse_k_group(list.head, 3),
            Some(Box::new(Node {
                data: 3,
                next: Some(Box::new(Node {
                    data: 2,
                    next: Some(Box::new(Node {
                        data: 1,
                        next: Some(Box::new(Node {
                            data: 4,
                            next: Some(Box::new(Node {
                                data: 5,
                                next: None
                            }))
                        }))
                    }))
                }))
            }))
        );
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        list.insert(1).insert(2).insert(3).insert(4).insert(5);

        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_delete_nth_from_end() {
        let mut list = LinkedList::new();
        list.insert(1).insert(2).insert(3).insert(4).insert(5);

        list.delete_nth_from_end(2);

        let mut right = LinkedList::new();
        right.insert(1).insert(2).insert(3).insert(5);
        assert_eq!(list, right);

        let mut list = LinkedList::new();
        list.insert(1);

        list.delete_nth_from_end(1);

        assert_eq!(list, LinkedList::new());

        let mut list = LinkedList::new();
        list.insert(1).insert(2);
        list.delete_nth_from_end(1);
        assert_eq!(
            list,
            LinkedList {
                head: Some(Box::new(Node {
                    data: 1,
                    next: None
                }))
            }
        );
    }
}
