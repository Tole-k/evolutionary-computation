from components import algorithm_comparison_page
from utils import Algorithm
from pages.regret import ALGORITHMS as regret_algorithms

INITIAL_RANDOM = r"""
```
n = len(all_points) + 1 // 2
initial solution = get_random_solution(n)

"""
INITIAL_GREEDY = r"""
```
n = len(all_points) + 1 // 2
initial solution = nn_to_any_regret(n, weights=[0.5, 0.5])

"""

NODES = r"""
fn intra(i, j):
    a = Distance[(i - 1) % n][i]
    b = Distance[(i - 1) % n][j]
    c = Distance[(j - 1) % n][i]
    d = Distance[(j - 1) % n][j]

    e = Distance[i][(i + 1) % n]
    f = Distance[j][(i + 1) % n]
    g = Distance[i][(j + 1) % n]
    h = Distance[j][(j + 1) % n]
    if i + 1 == j
        return b + g - a - h 
    else if (j + 1) % n == j
        return -e - d + f + c
    else
        return -a + b + c - d - e + f + g - h

fn intra_change(solution, i, j):
    solution = copy(solution)
    temp = -1
    solution.replace(i, a)
    solution.replace(j, i)
    solution.replace(a, j)
    return solution
"""

EDGES = r"""
fn intra(i, j):
    (a, b) = (i, j)
    if (j + 1) % n == j:
        (a, b) = (j, i)
    return Distance[a-1][a] - Distance[b][b+1] + Distance[a-1][b] + Distance[a][b+1]

fn intra_change(solution, i, j):
    solution = copy(solution)
    solution.replace(i, j)
    return solution
"""

GREEDY = r"""
fn inter(i, j):
    return -Distance[i-1][i] - Distance[i][i+1] - Cost[i]
        + Distance[i-1][j] + Distance[j][i+1] + Cost[j]

fn inter_change(solution, i, j):
    solution = copy(solution)
    solution.replace(i, j)
    return solution

while True:
    candidates = {}
    for i in 0 to n-1
        for j in i + 1 to n
            candidates.add(('intra', i, j))
    difference = all_points / solution 
    for i in solution:
        for j in difference
            candidates.add(('inter', i, j))
    shuffle(candidates)

    for (neighborhood, i, j) in candidates:
        if neighborhood == 'intra':
            delta = intra(i, j)
        else:
            delta = inter(i, j)
        if delta < 0:
            if neighborhood == 'intra':
                solution = intra_change(solution, i, j)
            else:
                solution = inter_change(solution, i, j)
            break
    
    if delta ≥ 0:
        break
```
"""

STEEPEST = r"""
fn inter(i, j):
    return -Distance[i-1][i] - Distance[i][i+1] - Cost[i]
        + Distance[i-1][j] + Distance[j][i+1] + Cost[j]

fn inter_change(solution, i, j):
    solution = copy(solution)
    solution.replace(i, j)
    return solution

while True:
    candidates = {}
    for i in 0 to n-1
        for j in i + 1 to n
            candidates.add(('intra', i, j))
    difference = all_points / solution 
    for i in solution:
        for j in difference
            candidates.add(('inter', i, j))
    shuffle(candidates)
    
    best_delta = INFINITY
    for (neighborhood, i, j) in candidates:
        if neighborhood == 'intra':
            delta = intra(i, j)
        else:
            delta = inter(i, j)
        if delta < best_delta:
            best_delta = delta
            if neighborhood == 'intra':
                new_solution = intra_change(solution, i, j)
            else:
                new_solution = inter_change(solution, i, j)
    
    if best_delta < 0:
        solution = new_solution
    else:
        break
```
"""

CONCLUSIONS = r"""
- Local Search algorithms provides reasonably good solution.
- If initial point is random, then it's is likely to end up in local optima very fast.
- Local searches starting from random solution take much longer time to converge.
- Difference between steepest and greedy approach is barely visible
- It doesn't take too much time to run local search when starting from non-radom solution, and because of guarantee of improvement
- (or at least not losing score) it's usually worth it to run
"""

ALGORITHMS = [
    Algorithm(
        "ls_greedy_edges_random",
        "ls_greedy_edges_random",
        INITIAL_RANDOM + EDGES + GREEDY,
    ),
    Algorithm(
        "ls_greedy_edges_greedy",
        "ls_greedy_edges_greedy",
        INITIAL_GREEDY + EDGES + GREEDY,
    ),
    Algorithm(
        "ls_greedy_nodes_random",
        "ls_greedy_nodes_random",
        INITIAL_RANDOM + NODES + GREEDY,
    ),
    Algorithm(
        "ls_greedy_nodes_greedy",
        "ls_greedy_nodes_greedy",
        INITIAL_GREEDY + NODES + GREEDY,
    ),
    Algorithm(
        "ls_steepest_edges_random",
        "ls_steepest_edges_random",
        INITIAL_RANDOM + EDGES + STEEPEST,
    ),
    Algorithm(
        "ls_steepest_edges_greedy",
        "ls_steepest_edges_greedy",
        INITIAL_GREEDY + EDGES + STEEPEST,
    ),
    Algorithm(
        "ls_steepest_nodes_random",
        "ls_steepest_nodes_random",
        INITIAL_RANDOM + NODES + STEEPEST,
    ),
    Algorithm(
        "ls_steepest_nodes_greedy",
        "ls_steepest_nodes_greedy",
        INITIAL_GREEDY + NODES + STEEPEST,
    ),
]


if __name__ == "__main__":
    algorithm_comparison_page(
        ALGORITHMS + regret_algorithms, "Local Search", conclusions=CONCLUSIONS
    )
