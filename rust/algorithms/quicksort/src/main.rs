// quicksort in rust
// ref https://www.hackertouch.com/quick-sort-in-rust.html
// ref https://gist.github.com/Triagle/96b9065bf240f94a0c0f

// quicksort creates a pivot and partitioning the list around it, where bigger elements are on
// one side and smaller on the other, and repeated until every element is sorted in
// each sublist

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        // if low is greater than high, the list is sorted
        let p = partition(arr, low, high); // partition the list
        _quick_sort(arr, low, p - 1); // recusive call to quicksort
        _quick_sort(arr, p + 1, high); // recusive call to quicksort
    }
}

fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize; // pivot is the last element
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
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