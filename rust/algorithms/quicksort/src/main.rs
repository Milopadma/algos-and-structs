// quicksort in rust
// ref https://www.hackertouch.com/quick-sort-in-rust.html
// ref https://gist.github.com/Triagle/96b9065bf240f94a0c0f

// quicksort creates a pivot and partitioning the list around it, where bigger elements are on
// one side and smaller on the other, and repeated until every element is sorted in
// each sublist

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    // type T must implement Ord because we are comparing elements
    // this function is a warpper for the _quick_sort function, as
    // _quick_sort takes two more arguments
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

// a private function that takes a mut slice of generic type
// this function is recursive
fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        // if low is greater than high, the list is sorted
        let p = partition(arr, low, high); // partition the list
        _quick_sort(arr, low, p - 1); // recusive call to quicksort
        _quick_sort(arr, p + 1, high); // recusive call to quicksort
    }
}

// a private function that partitions the mut array depending on the pivot element
fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize; // pivot is the last element
    let mut store_index = low - 1; // store_index is the index of the smaller element
    let mut last_index = high; // last_index is the last element

    loop {
        store_index += 1; // increment store_index
        while arr[store_index as usize] < arr[pivot] {
            // if the element is smaller than the pivot
            store_index += 1; // increment store_index, q:why? a:to find the next bigger element
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            // if the element is bigger than the pivot
            last_index -= 1; // decrement last_index, q:why? a:to find the next smaller element
        }
        if store_index >= last_index {
            break; // why break? a: because the list is sorted
        } else {
            arr.swap(store_index as usize, last_index as usize); // swap the elements
        }
    }
    arr.swap(store_index as usize, pivot as usize); // swap the pivot with the store_index because
    store_index
}

#[cfg(test)]
mod testing {
    #[test]
    fn test_1() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        super::quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

fn main() {
    let mut arr = [2, 1, 3, 4, 5, 6, 7, 8, 9];
    quick_sort(&mut arr);
    println!("{:?}", arr);
}