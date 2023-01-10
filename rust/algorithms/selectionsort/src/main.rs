use std::vec;

// selection sort in rust
// searches the list for the smallest element, then swaps it with the first element of this list, cuts off the first element and repeat
// ref https://applied-math-coding.medium.com/implementing-standard-algorithms-in-rust-bubble-sort-selection-sort-5c4cfb1a78c7
fn selection_sort<T: PartialOrd>(vector: &mut [T]){
    let (min_idx, _) = vector // get the index of the smallest element
    .iter() // get an iterator over the elements
    .enumerate() // to get the index of the elements
    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()) // compare the elements
    .unwrap(); // get the smallest element
    vector.swap(0, min_idx); // swap the smallest element with the first element
    // recursively call the function on the rest of the list
    if vector.len() > 1 {
        selection_sort(&mut vector[1..]);
    }
}

#[test]
fn test_selection_sort() {
    let mut vector = vec![5, 4, 3, 2, 1];
    selection_sort(&mut vector);
    assert_eq!(vector, vec![1, 2, 3, 4, 5]);
}


fn main() {
    let mut vector = vec![5, 4, 3, 2, 1];
    selection_sort(&mut vector);
    println!("{:?}", vector);
}