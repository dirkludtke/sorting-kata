import type { ListElement } from "./Element";


/**
 * sort a list of elements in ascending order with O(n * log(n)) maximum time complexity.
 * @param elements list of elements to sort (sorting will modify the list).
 */
function sort(elements: ListElement[]): void {
  // quicksort has a worst case of O(n**2). let's go with something like mergesort.
  if (elements.length <= 1) {
    return;
  }
  let mid = Math.floor(elements.length / 2);
  let left = elements.slice(0, mid);
  let right = elements.slice(mid);
  sort(left);
  sort(right);
  let [leftIndex, rightIndex] = [0, 0];
  for (let i = 0; i < elements.length; i++) {
    if (leftIndex == left.length) {
      elements[i] = right[rightIndex++];
    }
    else if (rightIndex == right.length) {
      elements[i] = left[leftIndex++];
    }
    else if (left[leftIndex].lessThan(right[rightIndex])) {
      elements[i] = left[leftIndex++];
    }
    else {
      elements[i] = right[rightIndex++];
    }
  }
}


export { sort };
