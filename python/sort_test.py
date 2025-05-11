import os
import json

import sort_execute


test_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'testdata')

def test(algorithm: str) -> None:
    """
    test a sorting algorithm with all test data sets until it fails.
    @param algorithm: the sorting algorithm to test.
    """
    print(f'testing {algorithm}...')

    # go through all test data set stages
    for data_stage in range(1, 6):
        # load test data (input and expected output is in separate files)
        input_name = os.path.join(test_dir, f'input_stage_{data_stage}.json')
        output_name = os.path.join(test_dir, f'output_stage_{data_stage}.json')
        print(f'  data sets of {input_name}...')
        input_list = sort_execute.load_data(os.path.join(test_dir, input_name))

        # check whether output data is available. abort if not.
        try:
            output_list = sort_execute.load_data(os.path.join(test_dir, output_name))
            assert isinstance(output_list, list) and len(output_list) == len(input_list)
        except Exception:
            assert False, f'Cannot test {input_name} because output data is missing (or incorrect)'
        # check all test data sets
        fail_count = 0
        for test_id, (input_data, expected) in enumerate(zip(input_list, output_list), 1):
            sort = sort_execute.sort_functions[algorithm]
            output, _, count_str = sort_execute.execute(sort, input_data)
            print(f"    tested case {test_id} {shorten(input_data)} in {count_str}")
            if output != expected:
                print(f"FAILED.\noutput   {output} !=\nexpected {expected}")
                fail_count += 1
        # abort if a test case failed
        assert fail_count == 0, f"{fail_count} test cases failed for stage {data_stage}."

class Threedots:
    def __repr__(self):
        return '...'
threedots = Threedots()

def shorten(data: list, max_length: int = 4) -> str:
    if len(data) > max_length:
        return str(data[:max_length] + [threedots]) + f' (length {len(data)})'
    return str(data)


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
