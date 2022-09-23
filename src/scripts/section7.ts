////////////////////////*
//*recursive functions//*
////////////////////////*

//recursive function to countdown from a number
export function countdown(num: number) {
  if (num <= 0) {
    console.log("All done!");
    return "done";
  }
  console.log(num);
  num--;
  countdown(num);
}

//recursive function to find the factorial of a number
export function factorial(num: number): number {
  if (num === 1) return 1;
  return num * factorial(num - 1);
}

//recursive function that collects odds
function collectOddValues(arr1: number[]): number[] {
  let result: number[] = [];
  function helper(helperInput: number[]) {
    if (helperInput.length === 0) {
      return;
    }
    if (helperInput[0] % 2 !== 0) {
      result.push(helperInput[0]);
    }
    helper(helperInput.slice(1));
  }
  helper(arr1);
  return result;
}

//pure recursive function that collects odds
function collectOddValuesPure(arr1: number[]): number[] {
  let newArr: number[] = [];
  if (arr1.length === 0) {
    return newArr;
  }
  if (arr1[0] % 2 !== 0) {
    newArr.push(arr1[0]);
  }
  newArr = newArr.concat(collectOddValuesPure(arr1.slice(1)));
  return newArr;
}

//pure recursive function that returns the power of the base to the exponent
function power(base: number, exponent: number): number {
  if (exponent === 0) return 1;
  return base * power(base, exponent - 1);
}

function main() {
  console.log("countdown from 5");
  console.log(countdown(5));
  console.log("factorial of 5");
  console.log(factorial(5)); // 120
  console.log("collect odd values from [1,2,3,4,5,6,7,8,9]");
  console.log(collectOddValues([1, 2, 3, 4, 5, 6, 7, 8, 9])); // [1,3,5,7,9]
  console.log("collect odd values from [1,2,3,4,5,6,7,8,9]");
  console.log(collectOddValuesPure([1, 2, 3, 4, 5, 6, 7, 8, 9])); // [1,3,5,7,9]
}

main();
