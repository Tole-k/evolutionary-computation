import streamlit as st
from components import algorithm_comparison_page
from utils import Algorithm, dill_cache
import evolutionary
from pages.local import ALGORITHMS as local_algorithms

INITIAL_RANDOM = r"""
```
n = len(all_points) + 1 // 2
initial solution = get_random_solution(n)

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

STEEPEST = r"""
fn inter(i, j):
    return -Distance[i-1][i] - Distance[i][i+1] - Cost[i]
        + Distance[i-1][j] + Distance[j][i+1] + Cost[j]

fn inter_change(solution, i, j):
    solution = copy(solution)
    solution.replace(i, j)
    return solution

fn build_candidates(Distance, n):
    top_n = []
    for i in 0 to length of Distance:
        sorted_neighbors = sort(Distance[i])
        top_n.add(sorted_neighbors[:n])
    return top_n

best_n_candidates = build_candidates(Distance, 10)

while True:
    candidates = {}
    for id, i in enumerate(solution):
        for candidate in best_n_candidates:
            if candidate in solution:
                candidates.add(('intra', solution[id-1], j))
                candidates.add(('intra', i, j))
                candidates.add(('intra', solution[id+1], j))
            else:
                candidates.add(('inter', solution[id-1], j))
                candidates.add(('inter', i, j))
                candidates.add(('inter', solution[id+1], j))

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
- We know that the full local search should be upper limit for our solution score (It doesn't have to be because of random start, and it can end up in different local optimum)
- Algorithm performs visibly faster, with little performance loss
- When the number of candidates is more than 60 than original implementation is faster, as creation of neighborhood and search if node is in initial solution takes significant time
- 10 candidates result is visibly worse solution, but because of very high speed up it is worth of consideration, nevertheless with 15+ candidates, the score difference is very little, especially if we take into consideration that the initial solution is random, making it better to run few more random run's rather than making full local search on fewer runs
"""

ALGORITHMS = [
    Algorithm(
        "LS Candidate 10",
        "ls_candidate_10_edge",
        INITIAL_RANDOM + EDGES + STEEPEST,
    ),
    Algorithm(
        "LS Candidate 25",
        "ls_candidate_25_edge",
        "",
    ),
    Algorithm(
        "LS Candidate 50",
        "ls_candidate_50_edge",
        "",
    ),
]


if __name__ == "__main__":

    @dill_cache(f"{st.session_state['tsp_version']}-candidates")
    def get_candidates_metrics():
        metrics = [
            (
                evolutionary.benchmark_candidates(
                    st.session_state["tsp_version"].replace(" ", ""), i
                ),
                print("Done"),
            )[0]
            for i in range(1, 50)
        ]
        scores = [metric.scores for metric in metrics]
        times = [metric.times for metric in metrics]
        return scores, times

    algorithm_comparison_page(
        ALGORITHMS,
        "Local Search Candidates",
        [local_algorithms[0], local_algorithms[4], local_algorithms[5]],
        CONCLUSIONS,
    )
    metric = evolutionary.main(
        st.session_state["tsp_version"].replace(" ", ""), ["ls_steepest_edges_random"]
    )[0]
    scores, times = get_candidates_metrics()
    col1, col2 = st.columns(2)
    s_time = sum(metric.times) / 200
    s_score = sum(metric.scores) / 200
    col1.subheader("Time")
    col1.line_chart(
        {
            "local_search_candidates": [sum(time) / 200 for time in times],
            "local_search": [s_time for _ in times],
        },
        x_label="number of candidates",
        y_label="time",
    )
    col2.subheader("Score")
    col2.line_chart(
        {
            "local_search_candidates": [sum(score) / 200 for score in scores],
            "local_search": [s_score for _ in times],
        },
        x_label="number of candidates",
        y_label="score",
    )
