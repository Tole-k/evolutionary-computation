from page_generator import algorithm_comparison_page, Algorithm

NN_TO_LAST_POINT_PSEUDOCODE = """
```
not_chosen_points = []
chosen_points = []
chosen_points.add(starting_point)
while chosen_points.length() < all_points.length() // 2:
    best_point = starting_point
    best_point_cost = INFINITY
    for point in not_chosen_points:
        cost = Distance_Matrix[chosen_points[-1]][point]
        if cost < best_point_cost:
            best_point = point
            best_point_cost = cost
    chosen_points.add(point)
    not_chosen_points.remove(point)
```
"""
NN_TO_ANY_POINT_PSEUDOCODE = r"""

"""
GREEDY_CYCLE_PSEUDOCODE = r"""

"""

CONCLUSIONS = r"""

"""


ALGORITHMS = [
    Algorithm("Random", "random", "... Random ???"),
    Algorithm("NN to last point", "nn_to_last_point", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("NN to any point", "nn_to_any_point", NN_TO_ANY_POINT_PSEUDOCODE),
    Algorithm("Greedy Cycle", "greedy_cycle", GREEDY_CYCLE_PSEUDOCODE),
]


if __name__ == "__main__":
    algorithm_comparison_page(ALGORITHMS, "Greedy algorithms", conclusions=CONCLUSIONS)
