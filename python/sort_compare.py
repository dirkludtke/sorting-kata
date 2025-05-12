import math
import random
import sort_execute


data_types = {
    'uniform': lambda length: [0 for _ in range(length)],
    'repeating': lambda length: [random.randrange(math.isqrt(length)) for i in range(length)],
    'random': lambda length: [random.randrange(length) for _ in range(length)],
    'unique': lambda length: random.sample(range(length), length),
    'ascending': lambda length: [i for i in range(length)],
    'descending': lambda length: [i for i in range(length - 1, -1, -1)],
}

test_sizes = {
    'tiny': 4,
    'small': 9, 
    'medium': 400,
    'large': 10000
}

def compare() -> None:
    """
    Compares the different sorting algorithm implementations and the Python list.sort function.
    The algorithms are executed with different data types and sizes and the results are printed.
    """
    # identify the algorithms ready for testing by executing a known test case
    sort_names = []
    for name, sort in sort_execute.sort_functions.items():
        try:
            output, _, _ = sort_execute.execute(sort, [3, 1, 4, 1, 5, 9, 2, 6, 5])
            assert output == [1, 1, 2, 3, 4, 5, 5, 6, 9]
            sort_names.append(name)
        except Exception:
            print(f"{name} is not ready for comparison.")

    # make a paragraph for each data type, a line for each algorithm and a column for each size
    # - for getting placement information, we execute all algortithms before printing
    for data_type, data_generator in data_types.items():
        data_type_str = f'{data_type} data (e.g. {', '.join(str(n) for n in data_generator(4))})'
        print(f'\nTesting {data_type_str} of sizes', ', '.join((str(i) for i in test_sizes.values())))
        sort_results = [[] for _ in sort_names]
        for size in test_sizes.values():
            random.seed(1) # ensure that it is always the same random data
            input_data = data_generator(size)
            expected = sorted(input_data)
            test_results = []
            for stage_id, sort_name in enumerate(sort_names):
                try:
                    sort = sort_execute.sort_functions[sort_name]
                    output, count, count_str = sort_execute.execute(sort, input_data, limit=3600*24*10)
                    if output != expected:
                        count, count_str = None, 'wrong result'
                except Exception as e:
                    count, count_str = None, 'time/error'
                sort_results[stage_id].append(f'{count_str:>10}')
                test_results.append(count)
            positions = get_positions(test_results)
            for stage_id in range(len(sort_names)):
                sort_results[stage_id][-1] += f' ({positions[stage_id]})'
        for stage_id, sort_name in enumerate(sort_names):
            print(f"{sort_name:8}: {', '.join(sort_results[stage_id])}")

def get_positions(scores: list[int | None]) -> list[int]:
    """
    determine the position of each algorthm (1) being the best. if there are multiple algorthims
    with the same score, they get the same position and the next positions are skipped.
    """
    score_ids = {}
    for i, score in enumerate(scores):
        score_ids.setdefault(score, []).append(i)
    pos = 1
    positions = [None] * len(scores)
    for score in sorted(score_ids.keys(), key=lambda score: (score is None, score)):
        for i in score_ids[score]:
            positions[i] = pos
        pos += len(score_ids[score])
    return positions


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description='''
        Compares the different sorting algorithm implementations and the Python list.sort function.
    ''')
    args = parser.parse_args()
    compare()
