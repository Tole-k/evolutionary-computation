from components import algorithm_comparison_page
from utils import Algorithm
from pages.local import ALGORITHMS as ls_algorithms

PSEUDOCODE = r""""""

CONCLUSIONS = r"""
- Scores are almost identical to those of the steepest local search algorithm with negligible deviations.
- Running times are vastly improved, resulting in a speedup of around 3x.
"""

ALGORITHMS = [
    Algorithm(
        "ls_cached_deltas_edges",
        "ls_cached_deltas_edges",
        PSEUDOCODE
    ),
    Algorithm(
        "ls_cached_deltas_nodes",
        "ls_cached_deltas_nodes",
        PSEUDOCODE
    )
]
if __name__ == "__main__":
    algorithm_comparison_page(
        ALGORITHMS, "Cached Deltas", [ls_algorithms[4], ls_algorithms[6]], conclusions=CONCLUSIONS
    )
