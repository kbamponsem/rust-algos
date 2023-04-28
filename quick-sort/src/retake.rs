fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    // Set the pivot index
    let pivot = low;

    let mut left = low;
    let mut right = high - 1;

    // Loop through the array by moving the left and right pointers
    // We will compare each item to pivot,
    // for left:
    // if arr[left] <= arr[pivot]
    // left ++;
    // for right:
    // if arr[right] > arr[pivot]
    // right --;

    // If left < right:
    // swap (arr[left], arr[right])

    // swap(arr[right], arr[pivot])
    // return right

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

    return right;
}
pub fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
    // We need to check if left < right
    if low >= high {
        return;
    }
    // Use partition to get the pivot
    let pivot = partition(arr, low, high);

    // Perform quick_sort on the left array
    quick_sort(arr, low, pivot);
    // Perform quick_sort on the right array
    quick_sort(arr, pivot + 1, high);
}
