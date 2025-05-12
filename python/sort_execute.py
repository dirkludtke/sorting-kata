import json
from typing import Callable, TextIO

from Element import Element
from Counter import Counter
import stage1
import stage2
import stage3
import stage4


sort_functions = {
    'stage1': stage1.sort,
    'stage2': stage2.sort,
    'stage3': stage3.sort,
    'stage4': stage4.sort,
    'pylist': lambda elements: elements.sort(),
}

def execute(
        sort: Callable[[list[Element]], None],
        input_data: list[int],
        limit=3600
) -> tuple[list[int], int, str]:
    """
    execute a sorting function with a test set.
    @param sort: the sorting function to test.
    @param input_data: the data to sort.
    """
    counter = Counter(limit=limit)
    input_elements = [Element(counter, i) for i in input_data]
    sort(input_elements)
    count, count_str = int(counter), str(counter)
    output = [int(e) for e in input_elements]
    return output, count, count_str

def load_data(path: str) -> list[list[int]]:
    """
    load the test data from a file.
    @param path: file path.
    """
    with open(path) as file:
        structure = json.load(file)
        assert isinstance(structure, list), f"Cannot use {path}. it is not a list."
        for i, item in enumerate(structure, start=1):
            assert isinstance(item, list) and all(isinstance(i, int) for i in item),\
                f"Cannot use {path} item {i}. it is not a list of integers."
    return structure

def save_data(path: str, data: list[list[int]]) -> None:
    """
    save the test (result) data to a file.
    @param path: file path.
    @param data: data to save.
    """
    with open(path, 'wt') as file:
        save_data_file(file, data)

def save_data_file(file: TextIO, data: list[list[int]]) -> None:
    """
    save the test data to an opened file.
    @param file: file to save to.
    @param data: data to save.
    """
    print('[', file=file)
    for i, item in enumerate(data, start=1):
        sep = ',' if i < len(data) else ''
        print('  ' + str(item) + sep, file=file)
    print(']', file=file)


if __name__ == '__main__':
    import argparse
    import sys
    parser = argparse.ArgumentParser(description='Execute a sort algorithm with some test data')
    parser.add_argument('algorithm', choices=sort_functions.keys(), help='algorithm to test')
    parser.add_argument(
        'data_path', help='path of test data (json list of test sets which are lists of integers)')
    args = parser.parse_args()
    try:
        input_data = load_data(args.data_path)
        output_data = []
        for data_set in input_data:
            sort_function = sort_functions[args.algorithm]
            output, _, _ = execute(sort_function, data_set)
            output_data.append(output)
        save_data_file(sys.stdout, output_data)
    except Exception as e:
        print('\n' + str(e), file=sys.stderr)
        exit(1)
