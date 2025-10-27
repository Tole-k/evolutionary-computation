from components import algorithm_comparison_page
from utils import Algorithm
from pages.regret import ALGORITHMS as regret_algorithms

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
    Algorithm("ls_greedy_edges_random", "ls_greedy_edges_random", RANDOM_PSEUDOCODE),
    Algorithm("ls_greedy_edges_greedy", "ls_greedy_edges_greedy", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("ls_greedy_nodes_random", "ls_greedy_nodes_random", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("ls_greedy_nodes_greedy", "ls_greedy_nodes_greedy", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("ls_steepest_edges_random", "ls_steepest_edges_random", RANDOM_PSEUDOCODE),
    Algorithm("ls_steepest_edges_greedy", "ls_steepest_edges_greedy", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("ls_steepest_nodes_random", "ls_steepest_nodes_random", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("ls_steepest_nodes_greedy", "ls_steepest_nodes_greedy", NN_TO_LAST_POINT_PSEUDOCODE),
]


if __name__ == "__main__":
    algorithm_comparison_page(ALGORITHMS + regret_algorithms, "Local Search", conclusions=CONCLUSIONS)
