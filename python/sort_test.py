import os
from typing import Callable

import sort_execute
from Element import Element


test_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'testdata')

def test(algorithm: str) -> None:
    """
    test a sorting algorithm with all test data sets until it fails.
    @param algorithm the sorting algorithm to test.
    """
    print(f'testing {algorithm}...')
    sort = sort_execute.sort_functions[algorithm]

    # go through all test data set stages
    for data_stage in range(1, 6):
        # load test data (input and expected output is in separate files)
        input_name = f'input_stage_{data_stage}.json'
        output_name = f'output_stage_{data_stage}.json'
        print(f'  data sets of {input_name}...')
        input_list = sort_execute.load_data(os.path.join(test_dir, input_name))

        # check whether output data is available. abort if not.
        try:
            output_list = sort_execute.load_data(os.path.join(test_dir, output_name))
            assert len(output_list) == len(input_list)
        except Exception:
            assert False, f'Cannot test {input_name} because output data is missing (or incorrect)'
        test_datasets(sort, input_list, output_list)

def test_datasets(
    sort: Callable[[list[Element]], None],
    input_list: list[list[int]],
    output_list: list[list[int]]
):
    fail_count = 0
    for test_id, (input_data, expected) in enumerate(zip(input_list, output_list), 1):
        output, _, count_str = sort_execute.execute(sort, input_data)
        print(f"    case {test_id} {listToString(input_data)} in {count_str}")
        if output != expected:
            output_str = listToString(output, -1)
            expected_str = listToString(expected, -1)
            print(f"FAILED.\noutput   {output_str} !=\nexpected {expected_str}")
            fail_count += 1
    # abort if a test case failed
    plural = 's' if fail_count != 1 else ''
    assert fail_count == 0, f"{fail_count} test case{plural} failed."

def listToString(data: list, max_length: int = 4) -> str:
    if max_length >= 0 and len(data) > max_length:
        return f'[{", ".join(str(n) for n in data[:max_length])}, ...] (length {len(data)})'
    return f'[{", ".join(str(n) for n in data)}]'


if __name__ == '__main__':
    import argparse
    import sys
    parser = argparse.ArgumentParser(
        description='Tests a sorting algortithm with all test data sets until it fails')
    choices = list(sort_execute.sort_functions.keys())
    parser.add_argument(
        'algorithm', nargs='?', default=choices[0], choices=choices, help='algorithm to test')
    args = parser.parse_args()
    try:
        test(args.algorithm)
    except Exception as e:
        print('\n' + str(e), file=sys.stderr)
        exit(1)
