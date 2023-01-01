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

//merge sort
export function mergeSort(arr: number[]): number[] {
  if (arr.length <= 1) return arr;
  let mid = Math.floor(arr.length / 2);
  let left = mergeSort(arr.slice(0, mid));
  let right = mergeSort(arr.slice(mid));
  return merge(left, right);
}

function merge(arr1: number[], arr2: number[]): number[] {
  let results: number[] = [];
  let i = 0;
  let j = 0;
  while (i < arr1.length && j < arr2.length) {
    if (arr2[j] > arr1[i]) {
      results.push(arr1[i]);
      i++;
    } else {
      results.push(arr2[j]);
      j++;
    }
  }
  while (i < arr1.length) {
    results.push(arr1[i]);
    i++;
  }
  while (j < arr2.length) {
    results.push(arr2[j]);
    j++;
  }
  return results;
}

function partition(arr: number[], start: number, end: number) {
  let pivot = arr[start];
  let swapIdx = start;
  for (let i = start + 1; i <= end; i++) {
    if (pivot > arr[i]) {
      swapIdx++;
      let temp = arr[swapIdx];
      arr[swapIdx] = arr[i];
      arr[i] = temp;
    }
  }
  let temp = arr[start];
  arr[start] = arr[swapIdx];
  arr[swapIdx] = temp;
  return swapIdx;
}

// quick sort
export function quickSort(
  arr: number[],
  left: number,
  right: number
): number[] {
  let pivot;
  let partitionIndex;
  if (left < right) {
    pivot = right;
    partitionIndex = partition(arr, left, right);
    quickSort(arr, left, partitionIndex - 1);
    quickSort(arr, partitionIndex + 1, right);
  }
  return arr;
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
