// ref found goldmine of data structures and algorithms in any language
// https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/README.md

fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i; // set j to i because we want to insert the element at i
        let current = arr[i]; // current is the element to be inserted

        while j > 0 && current < arr[j - 1] {
            // while the element to be inserted is less than the element at j - 1
            // this is where the "move" happens
            arr[j] = arr[j - 1]; // move the element at j - 1 to j
            j -= 1; // decrement j to avoid an infinite loop
        }

        arr[j] = current; // insert the element at j
    }
}

fn main() {
    let mut arr = [5, 4, 3, 2, 1];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}