use std::borrow::BorrowMut;

pub fn merge(mut left_arr: Vec<u32>, mut right_arr: Vec<u32>) -> Vec<u32> {
    let mut merged = Vec::new();

    while !left_arr.is_empty() && !right_arr.is_empty() {
        if let Some(left) = left_arr.first() {
            if let Some(right) = right_arr.first() {
                if left < right {
                    merged.push(*left);
                    *left_arr.borrow_mut() = left_arr.drain(1..).collect::<Vec<_>>();
                } else {
                    merged.push(*right);
                    *right_arr.borrow_mut() = right_arr.drain(1..).collect::<Vec<_>>();
                }
            }
        }
    }

    merged.append(&mut left_arr);
    merged.append(&mut right_arr);

    merged
}

pub fn merge_sort(arr: Vec<u32>) -> Vec<u32> {
    if arr.len() <= 1 {
        return arr;
    }
    let mid = arr.len() / 2;

    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let left = merge_sort(left);
    let right = merge_sort(right);

    return merge(left, right);
}

#[cfg(test)]
mod tests {
    use crate::merge_sort;
    use binary_search::binary_search;
    #[test]
    fn test_merge_sort() {
        let arr = vec![2, 5, 10, 1, 6];
        assert_eq!(merge_sort(arr.clone()), vec![1, 2, 5, 6, 10]);

        let arr = merge_sort(arr);
        assert!(binary_search(&arr[..], 6));

        let arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(merge_sort(arr.clone()), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}

fn main() {
    let arr = vec![2, 5, 10, 8, 6];

    println!("{:?}", merge_sort(arr));
}
