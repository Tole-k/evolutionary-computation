from statistics import mean
from components import algorithm_comparison_page
from utils import Algorithm
from pages.local import ALGORITHMS as local_algorithms
import pandas as pd
import numpy as np
import streamlit as st

from components.page_template import Algorithm
from components.tsp_plot import TSPPlotter
from problem import main
from utils import load_algorithm_results


MSLS = r"""
```
fn MSLS()
    best_solution = []
    best_score = INFINITY
    for _ in 200
        let starting_solution = generate_random_solution()
        let solution = local_search(starting_solution)
        let score = check_solution(solution)
        if score < best_score
            best_score = score
            best_solution = solution
    return best_solution
```
"""

ILS = r"""
```py
fn mix_solution(solution)
    n = solution.len()
    problem_size = data.len()
    new_solution = solution.clone()
    is_iter = random()
    if is_inter
        chosen_indices = (0..n)
        hash_solution = set.from(solution)
        all = set.from(0..problem_size)
        new_indices = all / hash_solution # Set operation
        chosen_indices.shuffle()
        new_indices.shuffle()
        for (i, j) in [chosen_indices, new_indices]
            new_solution[i] = j
    else
        random_start_point = random_range(0..n - 10)
        length = random_range(1..10)
        new_solution[random_start_point..random_start_point + length].shuffle()
    return new_solution

fn ils(solution, max_time)
    best_solution = generate_random_solution()
    best_score = INFINITY
    start_time = current_time()
    while start_time.elapsed() < max_time
        let solution = local_search(mix_solution(best_solution))
        let score = check_solution(solution)
        if score < best_score
            best_score = score
            best_solution = solution
    return best_solution
```
"""

CONCLUSIONS = r"""
- Multiple start local search is easier to implement and can be easily parallelized.
- Iterative local search gives the best results so far, beating local search with greedy start.
- It's harder to parallelize ILS.
"""

ALGORITHMS = [
    Algorithm(
        "Multiple start local search",
        "msls",
        MSLS,
    ),
    Algorithm(
        "Iterated local search",
        "ils",
        ILS,
    ),
]


def to_dataframe(solution_data):
    return pd.DataFrame(
        [
            [
                np.asarray(solution.scores).min(),
                np.asarray(solution.scores).mean(),
                np.asarray(solution.scores).max(),
                np.asarray(solution.times).min(),
                np.asarray(solution.times).mean(),
                np.asarray(solution.times).max(),
                np.asarray(solution.times).sum(),
            ]
            for solution in solution_data
        ],
        [solution.name for solution in solution_data],
        [
            "min score",
            "mean score",
            "max score",
            "min time [s]",
            "mean time [s]",
            "max time [s]",
            "total time [s]",
        ],
    )


if __name__ == "__main__":
    main(report=True)

    st.title("Multiple Start Local Search")
    tsp_plotter_a = TSPPlotter("TSP A", dark_mode=False)  # type: ignore
    tsp_plotter_b = TSPPlotter("TSP B", dark_mode=False)  # type: ignore
    import evolutionary

    counts_a, ils_scores_a, ils_path_a, msls_scores_a, msls_path_a = evolutionary.assignment_6("TSPA", 13, 5)
    counts_b, ils_scores_b, ils_path_b, msls_scores_b, msls_path_b = evolutionary.assignment_6("TSPB", 13, 5)
    for name, code, path_a, path_b in [("Ils", ILS, ils_path_a, ils_path_b), ("MSLS", MSLS, msls_path_a, msls_path_b)]:
        st.header(name)
        st.subheader("Pseudocode")
        st.markdown(code)
        st.subheader("TSP A")
        fig = tsp_plotter_a.plot(path_a)
        st.pyplot(fig)
        st.write(f"Best found path: {path_a}")
        st.subheader("TSP B")
        fig = tsp_plotter_b.plot(path_b)
        st.pyplot(fig)
        st.write(f"Best found path: {path_b}")

    df_a = pd.DataFrame(
        {
            "name": ["MSLS Scores", "ILS Scores", "MSLS Counts", "ILS Counts"],
            "min": [min(msls_scores_a), min(ils_scores_a), 200, min(counts_a)],
            "mean": [mean(msls_scores_a), mean(ils_scores_a), 200, mean(counts_a)],
            "max": [max(msls_scores_a), max(ils_scores_a), 200, max(counts_a)],
        }
    )
    st.header("TSP A")
    st.subheader("Scores")
    st.dataframe(df_a)

    df_b = pd.DataFrame(
        {
            "name": ["MSLS Scores", "ILS Scores", "MSLS Counts", "ILS Counts"],
            "min score": [min(msls_scores_b), min(ils_scores_b), 200, min(counts_a)],
            "mean score": [mean(msls_scores_b), mean(ils_scores_b), 200, mean(counts_a)],
            "max score": [max(msls_scores_b), max(ils_scores_b), 200, max(counts_a)],
        }
    )
    st.header("TSP B")
    st.subheader("Scores")
    st.dataframe(df_b)

    st.divider()
    st.subheader("Conclusions")
    st.markdown(CONCLUSIONS)
