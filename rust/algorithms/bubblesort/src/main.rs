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


fn main() {

}