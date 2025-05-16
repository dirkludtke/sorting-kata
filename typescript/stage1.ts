import type { ListElement } from "./Element";


/**
 * sort a list of elements in ascending order with O(n**2) time complexity.
 * @param elements list of elements to sort (sorting will modify the list).
 */
function sort(elements: ListElement[]): void {
  // even if you know faster algorithms, here, we want the simplest solution possible.
  // something like bubble sort or insertion sort is fine. maximum 6 lines of code.
  for (let i = 0; i < elements.length - 1; i++) {
    for (let j = i + 1; j < elements.length; j++) {
      if (elements[i].greaterThan(elements[j])) {
        [elements[i], elements[j]] = [elements[j], elements[i]];
      }
    }
  }
}


export { sort };
