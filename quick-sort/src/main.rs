fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = low;
    let mut left = low;
    let mut right = high - 1;

    while left < right {
        while arr[left] <= arr[pivot] {
            left += 1;
        }
        while arr[right] > arr[pivot] {
            right -= 1;
        }

        if left < right {
            let temp = arr[left];
            arr[left] = arr[right];
            arr[right] = temp;
        }
    }

    let temp = arr[pivot];
    arr[pivot] = arr[right];
    arr[right] = temp;

    right
}
fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let p_index = partition(arr, low, high);
        println!("P_Index: {p_index}");

        println!(
            "Left: {:?}\nRight: {:?}",
            &arr[low..p_index],
            &arr[p_index + 1..high]
        );
        quick_sort(arr, low, p_index);
        quick_sort(arr, p_index + 1, high);
    }
}

mod retake;

fn main() {
    println!("Hello, world!");
    let mut arr = vec![3, 2, 5, 4, 1];
    let high = arr.len();
    quick_sort(&mut arr, 0, high);
    println!("{arr:?}");
}

#[cfg(test)]
mod tests {
    use crate::quick_sort;
    use crate::retake;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![3, 2, 5, 4, 1];

        let high = arr.len();
        quick_sort(&mut arr, 0, high);
        assert_eq!(arr, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn test_retake_quick_sort() {
        let mut arr = vec![3, 2, 5, 4, 1];

        let high = arr.len();
        retake::quick_sort(&mut arr, 0, high);
        assert_eq!(arr, vec![1, 2, 3, 4, 5])
    }
}
