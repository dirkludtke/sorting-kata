from Element import Element


def sort(elements: list[Element]):
    """
    sort a list of elements in ascending order with O(n) maximum time complexity.
    @param elements: list of elements to sort (sorting will modify the list).
    """
    # yeah. you can do it. there is a hint in the documentation of the Element class.
    sorted_elements = [[] for _ in range(2**16)]
    for element in elements:
        value = int(element)
        sorted_elements[value].append(element)
    i = 0
    for equal_elements in sorted_elements:
        elements[i:i + len(equal_elements)] = equal_elements
        i += len(equal_elements)
