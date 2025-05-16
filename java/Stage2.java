class Stage2 implements Stage {

    /**
     * sort a list of elements in ascending order with O(n * log(n)) time complexity.
     * @param elements list of elements to sort (sorting will modify the list).
     */
    @Override
    public void sort(Element[] elements) {
        // this is a good time for quicksort. please use the first or last element as the pivot.
        // this is for didactic purposes as we want to explore where quicksort is not so quick.
        quicksort(elements, 0, elements.length - 1);
    }

    void quicksort(Element[] elements, int start, int end) {
        if (start >= end) {
            return;
        }
        Element pivot = elements[end];
        int index = start;
        for (int i = start; i < end; i++) {
            if (elements[i].lessThan(pivot)) {
                Element temp = elements[index];
                elements[index] = elements[i];
                elements[i] = temp;
                index++;
            }
        }
        Element temp = elements[index];
        elements[index] = elements[end];
        elements[end] = temp;
        quicksort(elements, start, index - 1);
        quicksort(elements, index + 1, end);
    }

}
