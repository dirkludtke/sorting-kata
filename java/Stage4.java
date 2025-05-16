import java.util.ArrayList;


class Stage4 implements Stage {

    /**
     * sort a list of elements in ascending order with O(n) maximum time complexity.
     * @param elements list of elements to sort (sorting will modify the list).
     */
    @Override
    public void sort(Element[] elements) {
        // yeah. you can do it. there is a hint in the documentation of the Element class.
        int length = 1 << 16;
        @SuppressWarnings("unchecked")
        ArrayList<Element>[] sortedElements = new ArrayList[length];
        for (int i = 0; i < length; i++) {
            sortedElements[i] = new ArrayList<Element>();
        }
        for (Element element : elements) {
            int value = element.toInteger();
            sortedElements[value].add(element);
        }
        int i = 0;
        for (int value = 0; value < length; value++) {
            for (Element element : sortedElements[value]) {
                elements[i++] = element;
            }
        }
    }

}
