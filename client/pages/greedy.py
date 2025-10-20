from components import algorithm_comparison_page
from utils import Algorithm

NN_TO_LAST_POINT_PSEUDOCODE = r"""```
closest(point, points, distance_matrix):
    closest_distance = infinity
    closest_point = point

    for each point in points:
        current_distance = distance_matrix[point, point] + point.cost

        if current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = point

    return closest_point, closest_distance



greedy_nn_to_last_point(points, starting_point, distance_matrix):
    last_point = starting_point
    path = [last_point]
    not_visited = points
    remove starting_point from not_visited

    for i from 1 to ceil(len(points) / 2):
        closest_point, _ = closest(last_point, not_visited, distance_matrix)

        append closest_point to path

        remove closest_point from not_visited

        last_point = closest_point

    return path
```"""
NN_TO_ANY_POINT_PSEUDOCODE = r"""```
closest(point, points, distance_matrix):
    closest_distance = INFINITY
    closest_point = point

    for each point in points:
        current_distance = distance_matrix[point, point] + point.cost

        if current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = point

    return closest_point, closest_distance


    
cheapest_ext(point_a, point_b, points, distance_matrix):
    closest_distance = infinity
    closest_point = point_a

    for each point in points:
        current_distance =
            distance_matrix[point_a, point] +
            distance_matrix[point, point_b] -
            distance_matrix[point_a, point_b] +
            point.cost

        if current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = point

    return closest_point, closest_distance



greedy_nn_to_any_point(points, starting_point, distance_matrix):
    path = [starting_point]
    not_visited = points
    remove starting_point from not_visited

    for i from 1 to ceil(len(points) / 2):
        insert_spot = 0
        closest_point = 0
        closest_distance = infinity

        for pos from 0 to len(path):

            if pos == 0:
                point, distance = closest(path[0], not_visited, distance_matrix)

            else if pos == len(path):
                point, distance = closest(path[len(path) - 1], not_visited, distance_matrix)

            else:
                a = path[pos - 1]
                b = path[pos]
                point, distance =
                    cheapest_ext(a, b, not_visited, distance_matrix)

            if distance < closest_distance:
                closest_distance = distance
                insert_spot = pos
                closest_point = point

        insert closest_point into path at insert_spot
        remove closest_point from not_visited

    return path
```"""
GREEDY_CYCLE_PSEUDOCODE = r"""```
cheapest_ext(point_a, point_b, points, distance_matrix):
    closest_distance = infinity
    closest_point = point_a

    for each point in points:
        current_distance =
            distance_matrix[point_a, point] +
            distance_matrix[point, point_b] -
            distance_matrix[point_a, point_b] +
            point.cost

        if current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = point

    return closest_point, closest_distance



greedy_cycle(points, starting_point, distance_matrix):
    path = [starting_point]
    not_visited = points
    remove starting_point from not_visited
    for i from 1 to ceil(len(points) / 2):
        insert_spot = 0
        closest_point = starting_point
        closest_distance = infinity
        for pos from 0 to len(path):
            if pos == 0 or pos == len(path):
                a = path[len(path) - 1]
                b = path[0]
            else:
                a = path[pos - 1]
                b = path[pos]

            point, distance = cheapest_ext(a, b, not_visited, distance_matrix)
            if distance < closest_distance:
                closest_distance = distance
                insert_spot = pos
                closest_point = point
        insert closest_point into path at insert_spot
        remove closest_point from not_visited
    return path
```"""
RANDOM_PSEUDOCODE = r"""
```
random (points)
    all_points = points
    all_points = shuffle (all_points)
    return all_points[:len(points)+1//2]
```
"""

CONCLUSIONS = r"""
- **Random algorithm** = the fastest, but gives terrible results.
- **NN to last point** = looks like typical greedy algorithm, visible not optimal, multiple crosses, but much better than random with ridiculously short time.
- **NN to any point** = performs usually worse than Greedy Cycle, with few runs, which got better results (It's possible as they are greedy algorithms, so they can make mistakes and not backtrack them).
- **Greedy cycle** = very similar to **NN to any point**, much longer time than **NN to last point**, but visibly better result.
"""

ALGORITHMS = [
    Algorithm("Random", "random", RANDOM_PSEUDOCODE),
    Algorithm("NN to last point", "nn_to_last_point", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("NN to any point", "nn_to_any_point", NN_TO_ANY_POINT_PSEUDOCODE),
    Algorithm("Greedy Cycle", "greedy_cycle", GREEDY_CYCLE_PSEUDOCODE),
]


if __name__ == "__main__":
    algorithm_comparison_page(ALGORITHMS, "Greedy algorithms", conclusions=CONCLUSIONS)
