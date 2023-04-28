use std::borrow::BorrowMut;

use crate::List;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Address {
    None,
    Some(Box<Node>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    data: i32,
    next: Address,
}

impl Node {
    pub fn new(data: i32) -> Self {
        Self {
            data,
            next: Address::None,
        }
    }

    pub fn insert(&mut self, data: i32) {
        match self.next {
            Address::None => {
                self.next = Address::Some(Box::new(Node::new(data)));
            }
            Address::Some(ref mut next_address) => next_address.insert(data),
        }
    }

    pub fn delete(&mut self, data: i32) {
        match self.next {
            Address::Some(ref mut next_address) => {
                if next_address.data == data {
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(data);
                }
            }
            Address::None => {
                println!("{data:?} not found in list");
                return;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinkedList {
    head: Address,
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
        Self {
            head: Address::None,
        }
    }

    pub fn insert(&mut self, data: i32) -> &mut Self {
        match self.head {
            Address::None => {
                self.head = Address::Some(Box::new(Node::new(data)));
            }
            Address::Some(ref mut next_address) => next_address.insert(data),
        }
        self
    }

    pub fn delete(&mut self, value: i32) -> &mut Self {
        match self.head {
            Address::None => {
                println!("List is empty");
            }
            Address::Some(ref mut next_address) => match next_address.next {
                Address::None => {
                    if next_address.data == value {
                        self.head = Address::None;
                    }
                }
                Address::Some(_) => next_address.delete(value),
            },
        }
        self
    }

    pub fn merge(mut left_arr: Vec<i32>, mut right_arr: Vec<i32>) -> Vec<i32> {
        let mut merged = Vec::new();

        while !left_arr.is_empty() && !right_arr.is_empty() {
            if let Some(left) = left_arr.first() {
                if let Some(right) = right_arr.first() {
                    if left < right {
                        merged.push(*left);
                        *left_arr.borrow_mut() = left_arr.drain(1..).collect::<Vec<_>>();
                        continue;
                    } else {
                        merged.push(*right);
                        *right_arr.borrow_mut() = right_arr.drain(1..).collect::<Vec<_>>();
                        continue;
                    }
                }
            }
        }

        merged.append(&mut left_arr);
        merged.append(&mut right_arr);

        merged
    }

    fn merge_sort(list: Vec<i32>) -> Vec<i32> {
        // Determine the mid point
        let mid = list.len() / 2;

        if list.len() <= 1 {
            return list;
        }
        let left = Self::merge_sort(list[..mid].to_vec());
        let right = Self::merge_sort(list[mid..].to_vec());

        return Self::merge(left, right);
    }
    pub fn merge_list(list: Vec<Vec<i32>>) -> Vec<i32> {
        // Merge all the list and then resort with merge sort

        let merged = list.iter().flatten().map(|v| *v).collect::<Vec<_>>();
        println!("{:?}", merged);

        Self::merge_sort(merged)
    }
    pub fn forward(&self) -> String {
        let mut node = &self.head;
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::simple::{Address, LinkedList, Node};

    #[test]
    fn test_merge_list() {
        let list = vec![vec![1, 2, 3], vec![1, 2], vec![4, 5, 6]];

        assert_eq!(LinkedList::merge_list(list), vec![1, 1, 2, 2, 3, 4, 5, 6])
    }

    #[test]
    fn test_insertion() {
        let mut list = LinkedList::new();

        list.insert(1).insert(2);

        assert_eq!(
            list,
            LinkedList {
                head: Address::Some(Box::new(Node {
                    data: 1,
                    next: Address::Some(Box::new(Node {
                        data: 2,
                        next: Address::None
                    }))
                }))
            }
        );
        println!("{list:?}");
    }
}
