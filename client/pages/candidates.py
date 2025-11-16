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
    for i in solution:
        for candidate in best_n_candidates:
            if candidate in solution:
                candidates.add(('intra', i, j))
            else:
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
- We know that the full local search is upped limit for our score
- Because of efficient neighborhood generation in version, the speed up wasn't so spectacular
- The idea of creating candidates to minimize the time necessary to achieve results faster would be great if the calculating cost would be higher
- The score on `n` below 50 is on average worse than from the classical local search
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
        ALGORITHMS + [local_algorithms[0], local_algorithms[4], local_algorithms[5]],
        "Local Search Candidates",
        conclusions=CONCLUSIONS,
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
