//////////////////////////*
//*searching algorithms//*
//////////////////////////*

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

//naive string search
export function naiveStringSearch(str1: string, str2: string) {
  let count = 0;
  for (let i = 0; i < str1.length; i++) {
    //loop through the first string
    for (let j = 0; j < str2.length; j++) {
      if (str2[j] !== str1[i + j]) {
        // and check if the character at the current index is equal to the first character of the second string
        break;
      }
      if (j === str2.length - 1) {
        //, if it is then loop through the second string and check if the character at the current index
        // is equal to the character at the current index of the first string,
        // if it is then increment the count
        count++;
      }
    }
  }
  return count;
}

function main() {
  console.log("linear search");
  console.log(linearSearch([1, 2, 3, 4, 5], 3)); // 2
  console.log("naive string search");
  console.log(naiveStringSearch("lorie loled", "lol")); // 1
}

main();
