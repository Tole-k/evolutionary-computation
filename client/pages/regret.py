from components import algorithm_comparison_page
from utils import Algorithm
from pages.greedy import ALGORITHMS as GREEDY_ALGORITHMS

NN_TO_ANY_2_REGRET_PSEUDOCODE = r"""```
nn_to_any_2_regret_pass(path, candidate_point, distance_matrix):
    best_pos = 0
    min_cost = infinity
    
    second_best_pos = 0
    second_min_cost = infinity
    
    for pos from 0 to length(path):
    
        if pos == 0:
            cost = distance_matrix[path[0], candidate_point] + candidate_point.cost

        else if pos == length(tsp_path):
            cost = distance_matrix[path[length(tsp_path) - 1], candidate_point] + candidate_point.cost

        else:
            a = tsp_path[pos - 1]
            b = tsp_path[pos]
            cost =
                distance_matrix[a, candidate_point] +
                distance_matrix[candidate_point, b] -
                distance_matrix[a, b] +
                candidate_point.cost

        if cost < min_cost:
            second_best_pos = best_pos;
            second_min_cost = min_cost;
            best_pos = pos
            min_cost = cost
        else if cost < second_min_cost:
            second_best_pos = pos
            second_min_cost = cost
    
    return best_pos, min_cost, second_min_cost - min_cost

nn_to_any_weighted_2_regret(points, starting_point, distance_matrix, weights):
    tsp_path = [starting_point]
    not_visited_points = points
    remove starting_point from not_visited_points

    for i from 1 to ceil(length(points) / 2):
        insert_spot = 0
        best_point = 0
        min_cost = infinity
        
        for each candidate_point in not_visited_points:
            pos, cost, regret = nn_to_any_2_regret_pass(tsp_path, candidate_point, distance_matrix)
            cost = cost * weights[0] - regret * weights[1]
            if cost < min_cost:
                min_cost = cost
                insert_spot = pos
                best_point = candidate_point
        
        insert closest_point into tsp_path at insert_spot
        remove closest_point from not_visited_points

    return tsp_path
```"""
GREEDY_CYCLE_2_REGRET_PSEUDOCODE = r"""```
greedy_cycle_2_regret_pass(path, candidate_point, distance_matrix):
    best_pos = 0
    min_cost = infinity
    
    second_best_pos = 0
    second_min_cost = infinity
    
    for pos from 0 to length(path):
    
        if pos == 0 or pos == length(tsp_path):
            a = tsp_path[length(tsp_path) - 1]
            b = tsp_path[0]
        else:
            a = tsp_path[pos - 1]
            b = tsp_path[pos]

        cost =
            distance_matrix[a, candidate_point] +
            distance_matrix[candidate_point, b] -
            distance_matrix[a, b] +
            candidate_point.cost

        if cost < min_cost:
            second_best_pos = best_pos;
            second_min_cost = min_cost;
            best_pos = pos
            min_cost = cost
        else if cost < second_min_cost:
            second_best_pos = pos
            second_min_cost = cost
    
    return best_pos, min_cost, second_min_cost - min_cost

greedy_cycle_weighted_2_regret(points, starting_point, distance_matrix, weights):
    tsp_path = [starting_point]
    not_visited_points = points
    remove starting_point from not_visited_points

    for i from 1 to ceil(length(points) / 2):
        insert_spot = 0
        best_point = 0
        min_cost = infinity
        
        for each candidate_point in not_visited_points:
            pos, cost, regret = greedy_cycle_2_regret_pass(tsp_path, candidate_point, distance_matrix)
            cost = cost * weights[0] - regret * weights[1]
            if cost < min_cost:
                min_cost = cost
                insert_spot = pos
                best_point = candidate_point
        
        insert closest_point into tsp_path at insert_spot
        remove closest_point from not_visited_points

    return tsp_path
```"""

CONCLUSIONS = r"""
- Performance of methods with pure 2-regret heuristics is abysmal. 
- 2-regret heuristics weighted with greedy methods (with 50%/50% weights) marginally outperform purely greedy methods.
- Since both algorithms have the same computational complexity their run time is very similar with greedy cycle being slightly faster in both variants.
- NN to any point gives on average slightly better results with weighted heuristic. while greedy cycle fares slightly better with pure 2 regret heuristic.
"""


ALGORITHMS = [
    Algorithm(
        "NN to any point 2 regret", "nn_to_any_2_regret", NN_TO_ANY_2_REGRET_PSEUDOCODE
    ),
    Algorithm(
        "NN to any point weighted 2 regret",
        "nn_to_any_weighted_2_regret",
        NN_TO_ANY_2_REGRET_PSEUDOCODE,
    ),
    Algorithm(
        "Greedy Cycle 2 regret",
        "greedy_cycle_2_regret",
        GREEDY_CYCLE_2_REGRET_PSEUDOCODE,
    ),
    Algorithm(
        "Greedy Cycle weighted 2 regret",
        "greedy_cycle_weighted_2_regret",
        GREEDY_CYCLE_2_REGRET_PSEUDOCODE,
    ),
]


if __name__ == "__main__":
    algorithm_comparison_page(
        ALGORITHMS,
        "Regret algorithms",
        additional_algorithms=GREEDY_ALGORITHMS,
        conclusions=CONCLUSIONS,
    )
