from components import algorithm_comparison_page
from utils import Algorithm

NN_TO_LAST_POINT_PSEUDOCODE = r"""```
find_closest(point, points, distance_matrix):
    closest_distance = infinity
    closest_point = point

    for each candidate_point in points:
        current_distance = distance_matrix[point, candidate_point]

        if current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = candidate_point

    return closest_point, closest_distance



greedy_nn_to_last_point(points, starting_point, distance_matrix):
    last_point = starting_point
    tsp_path = [last_point]
    not_visited_points = points
    remove starting_point from not_visited_points

    for i from 1 to ceil(length(points) / 2):
        closest_point, _ = find_closest(last_point, not_visited_points, distance_matrix)

        append closest_point to tsp_path

        remove closest_point from not_visited_points

        last_point = closest_point

    return tsp_path
```"""
NN_TO_ANY_POINT_PSEUDOCODE = r"""```
find_closest(point, points, distance_matrix):
    closest_distance = INFINITY
    closest_point = point

    for each candidate_point in points:
        current_distance = distance_matrix[point, candidate_point]

        if current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = candidate_point

    return closest_point, closest_distance


    
find_cheapest_extension(point_a, point_b, points, distance_matrix):
    closest_distance = infinity
    closest_point = point_a

    for each candidate_point in points:
        current_distance =
            distance_matrix[point_a, candidate_point] +
            distance_matrix[candidate_point, point_b] -
            distance_matrix[point_a, point_b]

        if current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = candidate_point

    return closest_point, closest_distance



greedy_nn_to_any_point(points, starting_point, distance_matrix):
    tsp_path = [starting_point]
    not_visited_points = points
    remove starting_point from not_visited_points

    for i from 1 to ceil(length(points) / 2):
        insert_spot = 0
        closest_point = 0
        closest_distance = infinity

        for pos from 0 to length(tsp_path):

            if pos == 0:
                point, distance = find_closest(tsp_path[0], not_visited_points, distance_matrix)

            else if pos == length(tsp_path):
                point, distance = find_closest(tsp_path[length(tsp_path) - 1], not_visited_points, distance_matrix)

            else:
                a = tsp_path[pos - 1]
                b = tsp_path[pos]
                point, distance =
                    find_cheapest_extension(a, b, not_visited_points, distance_matrix)

            if distance < closest_distance:
                closest_distance = distance
                insert_spot = pos
                closest_point = point

        insert closest_point into tsp_path at insert_spot
        remove closest_point from not_visited_points

    return tsp_path
```"""
GREEDY_CYCLE_PSEUDOCODE = r"""```
find_cheapest_extension(point_a, point_b, points, distance_matrix):
    closest_distance = infinity
    closest_point = point_a

    for each candidate_point in points:
        current_distance =
            distance_matrix[point_a, candidate_point] +
            distance_matrix[candidate_point, point_b] -
            distance_matrix[point_a, point_b]

        if current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = candidate_point

    return closest_point, closest_distance



greedy_cycle(points, starting_point, distance_matrix):
    tsp_path = [starting_point]
    not_visited_points = points
    remove starting_point from not_visited_points
    for i from 1 to ceil(length(points) / 2):
        insert_spot = 0
        closest_point = starting_point
        closest_distance = infinity
        for pos from 0 to length(tsp_path):
            if pos == 0 or pos == length(tsp_path):
                a = tsp_path[length(tsp_path) - 1]
                b = tsp_path[0]
            else:
                a = tsp_path[pos - 1]
                b = tsp_path[pos]

            point, distance = find_cheapest_extension(a, b, not_visited_points, distance_matrix)
            if distance < closest_distance:
                closest_distance = distance
                insert_spot = pos
                closest_point = point
        insert closest_point into tsp_path at insert_spot
        remove closest_point from not_visited_points
    return tsp_path
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
