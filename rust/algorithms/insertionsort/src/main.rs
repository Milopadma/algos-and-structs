// ref found goldmine of data structures and algorithms in any language
// https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/README.md

fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i; // j is the index of the element to be inserted
        let current = arr[i]; // current is the element to be inserted

        while j > 0 && current < arr[j - 1] {
            // while the element to be inserted is less than the element at j - 1
            arr[j] = arr[j - 1]; // move the element at j - 1 to j
            j -= 1; // decrement j
        }

        arr[j] = current; // insert the element at j
    }
}

fn main() {
    let mut arr = [5, 4, 3, 2, 1];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}