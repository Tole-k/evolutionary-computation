# flake8: noqa: PYI021
class Metrics:
    name: str
    scores: list[float]
    total_time: float
    best_solution: list[int]

    def __init__(self, name: str, scores: list[float], total_time: float, best_solution: list[int]) -> None: ...


def main(dataset: str, names: list[str]) -> list[Metrics]: ...
def complexity(dataset: str, name: str) -> list[float]: ...
