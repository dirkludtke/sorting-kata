from typing import Callable, Self


class Element:
    """
    a wrapper around integers which counts the number of comparison and value accesses.
    - it also ensures that integers are in a range from 0 to 65535 (uint16).
    """

    def __init__(self, counter: Callable, value: int):
        assert isinstance(value, int), f"Element value must be an integer, not {type(value)}"
        assert 0 <= value < 2**16, f"Element value must be uint16 (0-65535), not {value}"
        self._counter = counter
        self._value = value

    def __lt__(self, other: Self):
        return self._value < int(other)

    def __le__(self, other: Self):
        return self._value <= int(other)

    def __eq__(self, other: Self):
        return self._value == int(other)

    def __ge__(self, other: Self):
        return self._value >= int(other)

    def __gt__(self, other: Self):
        return self._value > int(other)

    def __int__(self):
        self._counter()
        return int(self._value)

    def __repr__(self):
        return str(self._value)
