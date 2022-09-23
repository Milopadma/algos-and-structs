// anagram pattern
export function validAnagram(str1: string, str2: string) {
  //check if the length of the strings are equal
  if (str1.length !== str2.length) {
    return false;
  }

  //use an object to store the frequency of each character in each string
  let frequencyCounterStr1: { [key: string]: number } = {};
  let frequencyCounterStr2: { [key: string]: number } = {};

  //loop through the first string and store the frequency of each character
  for (let char of str1) {
    frequencyCounterStr1[char] = (frequencyCounterStr1[char] || 0) + 1;
  }

  //loop through the second string and store the frequency of each character
  for (let char of str2) {
    frequencyCounterStr2[char] = (frequencyCounterStr2[char] || 0) + 1;
  }

  //then compare the two objects
  for (let key in frequencyCounterStr1) {
    if (!(key in frequencyCounterStr2)) {
      return false;
    }
    if (frequencyCounterStr1[key] !== frequencyCounterStr2[key]) {
      return false;
    }
  }
  return true;
}

//multiple pointers pattern
export function countUniqueValues(arr1: number[]) {
  // check if array is empty, if it is return 0
  // loop through the array and compare the index of i and j and if they are not equal then increment j and set the value of j to the value of i
  // return j + 1
  if (arr1.length === 0) {
    return 0;
  }
  let i = 0;
  for (let j = 1; j < arr1.length; j++) {
    //j starts off at second entry in array
    if (arr1[i] !== arr1[j]) {
      //if the value of i is not equal to the value of j
      i++;
      arr1[i] = arr1[j]; //set the value of i's array entry to the value of j's array entry
    }
  }
  return i + 1; //return the index of i + 1 because i starts off at 0
}

//sliding window pattern
export function maxSubarraySum(arr1: number[], num: number) {
  if (arr1.length < num) {
    // check if the length of the array is less than the number passed in, if it is return null
    return null;
  }
  let maxSum = 0; // a variable to store the max sum of the subarray
  let tempSum = 0; // a variable to store the temporary sum of the subarray
  for (let i = 0; i < num; i++) {
    // loop through the array and add the first num of entries to the temp sum
    maxSum += arr1[i];
  }
  tempSum = maxSum; // set the temp sum to the max sum because the first num of entries is the max sum
  for (let i = num; i < arr1.length; i++) {
    // loop through the array and subtract the first entry and add the next entry to the temp sum
    tempSum = tempSum - arr1[i - num] + arr1[i];
    maxSum = Math.max(maxSum, tempSum); // compare the temp sum to the max sum and set the max sum to the temp sum if it is greater
  }
  return maxSum;
}

//divide and conquer pattern (binary search)
export function binarySearch(arr1: number[], val: number) {
  // create a left pointer at the start of the array, and a right pointer at the end of the array
  let left = 0;
  let right = arr1.length - 1;
  let middle = Math.floor((left + right) / 2); // create a pointer in the middle
  console.log("middle is ", (middle+1));
  // while the left pointer comes before the right pointer:
  while (arr1[middle] !== val && left <= right) {
    if (val < arr1[middle]) {
      /// if the value is smaller than current index, move the right pointer down
      console.log("value is smaller, current middle is " + (middle +1) + " current right is " + (right+1));
      right = middle - 1;
      console.log("new right is " + (right+1));
    } else {
      /// if the value is larger than current index, move the left pointer up
      console.log("value is larger, current middle is " + (middle+1) + " current left is " + (left+1));
      left = middle + 1;
      console.log("new left is " + (left+1));
    }
    middle = Math.floor((left + right) / 2); // recalculate the middle
  }
  if (arr1[middle] === val) {
    /// if you find the value you want, return the index
    return middle;
  }
  return -1; // if you never find the value, return -1
}

function main() {
  console.log("validAnagram");
  console.log(validAnagram("anagram", "nagaram")); // true
  console.log(validAnagram("rat", "car")); // false
  console.log(validAnagram("qwerty", "qeywrt")); // true
  console.log("countUniqueValues");
  console.log(countUniqueValues([1, 2, 3, 4, 4, 4, 7, 7, 12, 12, 13])); // 7
  console.log(countUniqueValues([])); // 0
  console.log(countUniqueValues([-2, -1, -1, 0, 1])); // 4
  console.log("maxSubarraySum");
  console.log(maxSubarraySum([100, 200, 300, 400], 2)); // 700
  console.log(maxSubarraySum([1, 4, 2, 10, 23, 3, 1, 0, 20], 4)); // 39
  console.log(maxSubarraySum([-3, 4, 0, -2, 6, -1], 2)); // 5
  console.log("binarySearch");
  console.log(
    binarySearch(
      [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
      8
    )
  ); // 6
  // console.log(binarySearch([1, 2, 3, 4, 5], 3)); // 2
  // console.log(binarySearch([1, 2, 3, 4, 5], 5)); // 4
}

main();
