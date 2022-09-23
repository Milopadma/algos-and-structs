////////////////////////*
//* Sorting Algorithms //*
////////////////////////*

// Bubble Sort
export function bubbleSort(array: number[]): number[] {
  let noSwaps: boolean; // variable to keep track of whether or not there were any swaps
  for (let i = array.length; i > 0; i--) {
    // loop through the array as many times as there are elements in the array
    noSwaps = true;
    for (let j = 0; j < i - 1; j++) {
      // loop through the array and compare the
      //value of the current index to the value of the next index
      if (array[j] > array[j + 1]) {
        let temp = array[j];
        array[j] = array[j + 1]; // if the value of the current index is
        //greater than the value of the next index then swap the values
        array[j + 1] = temp;
        noSwaps = false;
      }
    }
    if (noSwaps) break; // if there were no swaps then break out of the loop
  }
  return array;
}

// Selection Sort
export function selectionSort(array: number[]): number[] {
  for (let i = 0; i < array.length; i++) {
    let lowest = i;
    for (let j = i + 1; j < array.length; j++) {
      if (array[j] < array[lowest]) {
        lowest = j;
      }
    }
    if (i !== lowest) {
      let temp = array[i];
      array[i] = array[lowest];
      array[lowest] = temp;
    }
  }
  return array;
}

// Insertion Sort
export function insertionSort(array: number[]): number[] {
  for (let i = 1; i < array.length; i++) {
    let currentVal = array[i];
    for (var j = i - 1; j >= 0 && array[j] > currentVal; j--) {
      array[j + 1] = array[j];
    }
    array[j + 1] = currentVal;
  }
  return array;
}

function main() {
  console.log("bubble sort");
  console.log(bubbleSort([37, 45, 29, 8])); // [8, 29, 37, 45]
  console.log("selection sort");
  console.log(selectionSort([37, 45, 29, 8])); // [8, 29, 37, 45]
  console.log("insertion sort");
  console.log(insertionSort([37, 45, 29, 8])); // [8, 29, 37, 45]
}

main();
