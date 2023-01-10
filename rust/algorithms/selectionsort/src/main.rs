use std::vec;

// selection sort in rust
// searches the list for the smallest element, then swaps it with the first element of this list, cuts off the first element and repeat
// ref https://applied-math-coding.medium.com/implementing-standard-algorithms-in-rust-bubble-sort-selection-sort-5c4cfb1a78c7
fn selection_sort<T: PartialOrd>(vector: &mut [T]){
    let (min_idx, _) = vector // get the index of the smallest element
    .iter()
    .enumerate()
    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
    .unwrap();
    vector.swap(0, min_idx);
    if vector.len() > 1 {
        selection_sort(&mut vector[1..]);
    }
}


fn main() {

}