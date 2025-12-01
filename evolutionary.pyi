# flake8: noqa: PYI021
class Metrics:
    name: str
    scores: list[float]
    times: list[float]
    best_solution: list[int]

    def __init__(
        self,
        name: str,
        scores: list[float],
        times: list[float],
        best_solution: list[int],
    ) -> None: ...

def main(dataset: str, names: list[str]) -> list[Metrics]: ...
def main_mc(dataset: str, names: list[str]) -> list[Metrics]: ...
def assignment_4(dataset: str, n: int) -> Metrics: ...
def complexity(dataset: str, name: str) -> list[float]: ...
def solution_history(dataset: str, name: str, point: int) -> list[list[int]]: ...
def run(dataset: str, name: str) -> Metrics: ...
def assignment_6(
    dataset: str, inter_mixin_size: int, intra_mixin_size: int
) -> (list[int], list[float], list[int], list[float], list[int]): ...  # type: ignore
