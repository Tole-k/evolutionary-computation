from page_generator import algorithm_comparison_page, Algorithm

NN_TO_LAST_POINT_PSEUDOCODE = r"""

"""
NN_TO_ANY_POINT_PSEUDOCODE = r"""

"""
GREEDY_CYCLE_PSEUDOCODE = r"""

"""

ALGORITHMS = [
    # Algorithm("Random", "random", "... Random ???"),
    Algorithm("NN to last point", "nn_to_last_point", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("NN to any point", "nn_to_any_point", NN_TO_ANY_POINT_PSEUDOCODE),
    Algorithm("Greedy Cycle", "greedy_cycle", GREEDY_CYCLE_PSEUDOCODE),
]


if __name__ == "__main__":
    algorithm_comparison_page(ALGORITHMS, "Greedy algorithms")
