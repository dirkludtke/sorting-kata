from Element import Element


def sort(elements: list[Element]):
    """
    sort a list of elements in ascending order with O(n * log(n)) time complexity.
    @param elements: list of elements to sort (sorting will modify the list).
    """
    # this is a good time for quicksort. please use the first or last element as the pivot.
    # this is for didactic purposes as we want to explore where quicksort is not so quick.
    quicksort(elements, 0, len(elements) - 1)

def quicksort(elements: list[Element], start: int, end: int):
    if start >= end:
        return
    pivot = elements[end]
    index = start
    for i in range(start, end):
        if elements[i] < pivot:
            elements[index], elements[i] = elements[i], elements[index]
            index += 1
    elements[index], elements[end] = elements[end], elements[index]
    quicksort(elements, start, index - 1)
    quicksort(elements, index + 1, end)
