from page_generator import algorithm_comparison_page, Algorithm

NN_TO_LAST_POINT_PSEUDOCODE = r"""```
find_closest(point, points, distance_matrix):
    closest_distance = INFINITY
    closest_point = point

    FOR each candidate_point IN points:
        current_distance = distance_matrix[point, candidate_point]

        IF current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = candidate_point

    RETURN (closest_point, closest_distance)



greedy_nn_to_last_point(points, starting_point, distance_matrix):
    last_point = starting_point
    tsp_path = [last_point]
    not_visited_points = points
    REMOVE starting_point from not_visited_points

    FOR i FROM 1 TO CEIL(length(points) / 2):
        (closest_point, _) = find_closest(last_point, not_visited_points, distance_matrix)

        APPEND closest_point TO tsp_path

        REMOVE closest_point from not_visited_points

        last_point = closest_point

    RETURN tsp_path
```"""
NN_TO_ANY_POINT_PSEUDOCODE = r"""```
find_closest(point, points, distance_matrix):
    closest_distance = INFINITY
    closest_point = point

    FOR each candidate_point IN points:
        current_distance = distance_matrix[point, candidate_point]

        IF current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = candidate_point

    RETURN (closest_point, closest_distance)


    
find_cheapest_extension(point_a, point_b, points, distance_matrix):
    closest_distance = INFINITY
    closest_point = point_a

    FOR each candidate_point IN points:
        current_distance =
            distance_matrix[point_a, candidate_point] +
            distance_matrix[candidate_point, point_b] -
            distance_matrix[point_a, point_b]

        IF current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = candidate_point

    RETURN (closest_point, closest_distance)



greedy_nn_to_any_point(points, starting_point, distance_matrix):
    tsp_path = [starting_point]
    not_visited_points = points
    REMOVE starting_point from not_visited_points

    FOR i FROM 1 TO CEIL(length(points) / 2):
        insert_spot = 0
        closest_point = 0
        closest_distance = INFINITY

        FOR pos FROM 0 TO length(tsp_path):

            IF pos == 0:
                (point, distance) =
                    find_closest(tsp_path[0], not_visited_points, distance_matrix)

            ELSE IF pos == length(tsp_path):
                (point, distance) =
                    find_closest(tsp_path[length(tsp_path) - 1],
                                 not_visited_points,
                                 distance_matrix)

            ELSE:
                a = tsp_path[pos - 1]
                b = tsp_path[pos]
                (point, distance) =
                    find_cheapest_extension(a, b, not_visited_points, distance_matrix)

            IF distance < closest_distance:
                closest_distance = distance
                insert_spot = pos
                closest_point = point

        INSERT closest_point INTO tsp_path AT insert_spot
        REMOVE closest_point from not_visited_points

    RETURN tsp_path
```"""
GREEDY_CYCLE_PSEUDOCODE = r"""```
find_cheapest_extension(point_a, point_b, points, distance_matrix):
    closest_distance = INFINITY
    closest_point = point_a

    FOR each candidate_point IN points:
        current_distance =
            distance_matrix[point_a, candidate_point] +
            distance_matrix[candidate_point, point_b] -
            distance_matrix[point_a, point_b]

        IF current_distance < closest_distance:
            closest_distance = current_distance
            closest_point = candidate_point

    RETURN (closest_point, closest_distance)



greedy_cycle(points, starting_point, distance_matrix):
    tsp_path = [starting_point]
    not_visited_points = points
    REMOVE starting_point from not_visited_points
    FOR i FROM 1 TO CEIL(length(points) / 2):
        insert_spot = 0
        closest_point = starting_point
        closest_distance = INFINITY
        FOR pos FROM 0 TO length(tsp_path):
            IF pos == 0 OR pos == length(tsp_path):
                a = tsp_path[length(tsp_path) - 1]
                b = tsp_path[0]
            ELSE:
                a = tsp_path[pos - 1]
                b = tsp_path[pos]

            (point, distance) = find_cheapest_extension(a, b, not_visited_points, distance_matrix)
            IF distance < closest_distance:
                closest_distance = distance
                insert_spot = pos
                closest_point = point
        INSERT closest_point INTO tsp_path AT insert_spot
        REMOVE closest_point from not_visited_points
    RETURN tsp_path
```"""

ALGORITHMS = [
    Algorithm("Random", "random", "... Random ???"),
    Algorithm("NN to last point", "nn_to_last_point", NN_TO_LAST_POINT_PSEUDOCODE),
    Algorithm("NN to any point", "nn_to_any_point", NN_TO_ANY_POINT_PSEUDOCODE),
    Algorithm("Greedy Cycle", "greedy_cycle", GREEDY_CYCLE_PSEUDOCODE),
]


if __name__ == "__main__":
    algorithm_comparison_page(ALGORITHMS, "Greedy algorithms")
