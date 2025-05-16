from Element import Element


def sort(elements: list[Element]):
    """
    sort a list of elements in ascending order with O(n * log(n)) maximum time complexity.
    @param elements: list of elements to sort (sorting will modify the list).
    """
    # quicksort has a worst case of O(n**2). let's go with something like mergesort.
    if len(elements) <= 1:
        return
    mid = len(elements) // 2
    left, right = elements[:mid], elements[mid:]
    sort(left)
    sort(right)
    left_index, right_index = 0, 0
    for i in range(len(elements)):
        if left_index == len(left):
            elements[i] = right[right_index]
            right_index += 1
        elif right_index == len(right):
            elements[i] = left[left_index]
            left_index += 1
        elif left[left_index] < right[right_index]:
            elements[i] = left[left_index]
            left_index += 1
        else:
            elements[i] = right[right_index]
            right_index += 1
