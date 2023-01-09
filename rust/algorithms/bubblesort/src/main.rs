// bubble sort implementation in rust

fn bubble_sort<T: PartialOrd>(vector: &mut Vec<T>) {
    let mut swapped = false; // flag to check if any swap has been made
    for i in 0..vector.len() - 1 {
        // iterate over the vector
        if vector[i] > vector[i + 1] {
            // check if the current element is greater than the next element
            vector.swap(i, i + 1); // swap the elements of the indexs
            swapped = true; // set the flag to true
        }
    }
    if swapped {
        bubble_sort(vector) // if any swap has been made, call the function again
    }
}

#[cfg(test)]
mod tests {
    // test module
    use super::*; // quick import of the bubble_sort function

    #[test]
    fn test_bubble_sort_1() {
        let mut vector = vec![0, 2, 3, 1, 4, 1];
        bubble_sort(&mut vector);
        assert_eq!(vector, vec![0, 1, 1, 2, 3, 4]);
    }
}
fn main() {
    let mut vector = vec![0, 2, 3, 1, 4, 1];
    bubble_sort(&mut vector);
    println!("{:?}", vector);
}

// ref https://applied-math-coding.medium.com/implementing-standard-algorithms-in-rust-bubble-sort-selection-sort-5c4cfb1a78c7