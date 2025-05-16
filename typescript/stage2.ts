import type { ListElement } from "./Element";


/**
 * sort a list of elements in ascending order with O(n * log(n)) time complexity.
 * @param elements list of elements to sort (sorting will modify the list).
 */
function sort(elements: ListElement[]): void {
  // this is a good time for quicksort. please use the first or last element as the pivot.
  // this is for didactic purposes as we want to explore where quicksort is not so quick.
  quicksort(elements, 0, elements.length - 1);
}

function quicksort(elements: ListElement[], start: number, end: number): void {
  if (start >= end) {
    return;
  }
  let pivot = elements[end];
  let index = start;
  //console.log('quicksort', start, end);
  for (let i = start; i < end; i++) {
    if (elements[i].lessThan(pivot)) {
      [elements[index], elements[i]] = [elements[i], elements[index]];
      index++;
    }
  }
  [elements[index], elements[end]] = [elements[end], elements[index]];
  quicksort(elements, start, index - 1);
  quicksort(elements, index + 1, end);
}


export { sort };
