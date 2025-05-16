import type { ListElement } from "./Element";


/**
 * sort a list of elements in ascending order with O(n) maximum time complexity.
 * @param elements list of elements to sort (sorting will modify the list).
 */
function sort(elements: ListElement[]): void {
  // yeah. you can do it. there is a hint in the documentation of the Element class.
  let length = 2 ** 16;
  let sortedElements: ListElement[][] = [];
  for (let i = 0; i < length; i++) {
    sortedElements[i] = [];
  }
  for (let element of elements) {
    let value = element.toInteger();
    sortedElements[value].push(element);
  }
  let i = 0;
  for (let equalElements of sortedElements) {
    elements.splice(i, equalElements.length, ...equalElements);
    i += equalElements.length;
  }
}


export { sort };
