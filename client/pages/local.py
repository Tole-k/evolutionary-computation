from components import algorithm_comparison_page
from utils import Algorithm

NN_TO_LAST_POINT_PSEUDOCODE = r"""
```
```
"""

GREEDY_CYCLE_PSEUDOCODE = r"""
```
```
"""

RANDOM_PSEUDOCODE = r"""
```
```
"""

CONCLUSIONS = r"""
"""

ALGORITHMS = [
    Algorithm("Random", "random", RANDOM_PSEUDOCODE),
    Algorithm("NN to last point", "nn_to_last_point", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("NN to any point", "nn_to_any_point", NN_TO_ANY_POINT_PSEUDOCODE),
    Algorithm("Greedy Cycle", "greedy_cycle", GREEDY_CYCLE_PSEUDOCODE),
    Algorithm("Random", "random", RANDOM_PSEUDOCODE),
    Algorithm("NN to last point", "nn_to_last_point", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("NN to any point", "nn_to_any_point", NN_TO_ANY_POINT_PSEUDOCODE),
    Algorithm("Greedy Cycle", "greedy_cycle", GREEDY_CYCLE_PSEUDOCODE),
]


if __name__ == "__main__":
    algorithm_comparison_page(ALGORITHMS, "Greedy algorithms", conclusions=CONCLUSIONS)
