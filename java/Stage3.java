import java.util.Arrays;


class Stage3 implements Stage {

    /**
     * sort a list of elements in ascending order with O(n * log(n)) maximum time complexity.
     * @param elements list of elements to sort (sorting will modify the list).
     */
    @Override
    public void sort(Element[] elements) {
        // quicksort has a worst case of O(n**2). let's go with something like mergesort.
        if (elements.length <= 1) {
            return;
        }
        int mid = elements.length / 2;
        Element[] left = Arrays.copyOfRange(elements, 0, mid);
        Element[] right = Arrays.copyOfRange(elements, mid, elements.length);

        this.sort(left);
        this.sort(right);
        int leftIndex = 0;
        int rightIndex = 0;
        for (int i = 0; i < elements.length; i++) {
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

}
