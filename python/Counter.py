class Counter:
    """
    counter of the comparison operations which pretends that one comparion takes 1 seconds.
    """

    def __init__(self, limit: int = 3600):
        self.count = 0
        self.limit = limit

    def __call__(self):
        self.count += 1
        assert self.count < self.limit,\
            f'Abort at count {self.count}. that corresponds to {self}!'

    def __int__(self) -> int:
        return self.count

    def __str__(self) -> str:
        count = self.count
        if count < 60:
            return f"{count} seconds" if count != 1 else "1 second"
        count = (count + 30) // 60
        if count < 60:
            return f"{count} minutes" if count != 1 else "1 minute"
        count = (count + 30) // 60
        if count < 24:
            return f"{count} hours" if count != 1 else "1 hour"
        count = (count + 12) // 24
        if count < 366:
            return f"{count} days" if count != 1 else "1 day"
        count = (count + 183) // 366
        return f"{count} years" if count != 1 else "1 year"
