////////////////////////
//searching algorithms//
////////////////////////

//linear search
export function linearSearch(arr1: number[], val: number) {
  //loop through the array and check if the value at the current index is equal to the value passed in, if it is return the index
  for (let i = 0; i < arr1.length; i++) {
    if (arr1[i] === val) {
      return i;
    }
  }
  return -1;
}

function main() {
  console.log("linear search");
  console.log(linearSearch([1, 2, 3, 4, 5], 3)); // 2
}

main();
