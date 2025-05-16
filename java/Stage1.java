class Stage1 implements Stage {

    /**
     * sort a list of elements in ascending order with O(n**2) time complexity.
     * @param elements list of elements to sort (sorting will modify the list).
     */
    @Override
    public void sort(Element[] elements) {
        // even if you know faster algorithms, here, we want the simplest solution possible.
        // something like bubble sort or insertion sort is fine. maximum 6 lines of code.
        for (int i = 0; i < elements.length - 1; i++) {
            for (int j = i + 1; j < elements.length; j++) {
                if (elements[i].greaterThan(elements[j])) {
                    Element temp = elements[i];
                    elements[i] = elements[j];
                    elements[j] = temp;
                }
            }
        }
    }

}
