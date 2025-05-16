from Element import Element


def sort(elements: list[Element]):
    """
    sort a list of elements in ascending order with O(n**2) time complexity.
    @param elements: list of elements to sort (sorting will modify the list).
    """
    # even if you know faster algorithms, here, we want the simplest solution possible.
    # something like bubble sort or insertion sort is fine. maximum 6 lines of code.
    for i in range(len(elements) - 1):
        for j in range(i + 1, len(elements)):
            if elements[i] > elements[j]:
                elements[i], elements[j] = elements[j], elements[i]
