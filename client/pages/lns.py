import json
from statistics import mean
from utils import Algorithm, load_algorithm_results
import pandas as pd
import numpy as np
import streamlit as st

from components.tsp_plot import TSPPlotter
from problem import main

LNS = r"""
```py
fn roulette_wheel_destroy(solution)
    n = solution.len()
    to_remove = ceil(n*0.4)
    for _ from 0 to to_remove
        random = random(0,solution.total_node_cost())
        accumulated = 0
        i=0
        while true
            accumulated += cost_of_node(solution[i])
            if accumulated > random
                break
            i += 1
        solution.remove(i)
    return solution

fn repair(solution)
    return nn_2_any_weighted_regret(solution, weights=[0.5,0.5])

fn lns(solution, max_time)
    best_solution = generate_random_solution()
    best_solution = local_search(best_solution)
    best_score = INFINITY
    start_time = current_time()
    while start_time.elapsed() < max_time
        solution = roulette_wheel_destroy(best_solution)
        solution = repair(solution)
        score = check_solution(solution)
        if score < best_score
            best_score = score
            best_solution = solution
    return best_solution
```
"""

LNS_LS = r"""
```py
fn roulette_wheel_destroy(solution)
    n = solution.len()
    to_remove = ceil(n*0.4)
    for _ from 0 to to_remove
        random = random(0,solution.total_node_cost())
        accumulated = 0
        i=0
        while true
            accumulated += cost_of_node(solution[i])
            if accumulated > random
                break
            i += 1
        solution.remove(i)
    return solution

fn repair(solution)
    return nn_2_any_weighted_regret(solution, weights=[0.5,0.5])

fn lns(solution, max_time)
    best_solution = generate_random_solution()
    best_solution = local_search(best_solution)
    best_score = INFINITY
    start_time = current_time()
    while start_time.elapsed() < max_time
        solution = roulette_wheel_destroy(best_solution)
        solution = repair(solution)
        solution = local_search(solution)
        score = check_solution(solution)
        if score < best_score
            best_score = score
            best_solution = solution
    return best_solution
```
"""

CONCLUSIONS = r"""
 - Tested 20%, 30%, and 40% removal rates, with 40% yielding marginally better results.
 - Both methods come really close to the ILS in terms of average solution quality.
 - LNS with local search on average gets slightly better than LNS without local search.
 - LNS without local search is significantly faster (has higher counts per run) and its running time is pretty much constant
 - LNS with local search run times are more varied due to local search component.
"""

ALGORITHMS = [
    Algorithm(
        "Large Neighborhood Search with Local Search",
        "large_neighborhood_search_w_ls",
        LNS_LS,
    ),
    Algorithm(
        "Large Neighborhood Search without Local Search",
        "large_neighborhood_search_wo_ls",
        LNS,
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

    st.title("Large Neighborhood Search")
    import evolutionary

    for ds in ["TSPA", "TSPB"]:
        tsp_plotter = TSPPlotter("TSP A" if ds=="TSPA" else "TSP B", dark_mode=False)
        try:
            with open('lns_result.json', 'r', encoding="UTF-8") as file:
                data = json.load(file)
                lns, lns_w_ls = data[ds]['LNS without LS'], data[ds]['LNS with LS']
                counts, scores, path = lns
                counts_ls, scores_ls, path_ls = lns_w_ls
                print("loading results")

        except (FileNotFoundError, KeyError):
            lns, lns_w_ls = evolutionary.assignment_7(ds, 0.4)
            _, (counts, scores, path) = lns
            _, (counts_ls, scores_ls, path_ls) = lns_w_ls
            print("saved results not found calculating new ones")
        
        for name, code, path in [
            ("Large Neighborhood Search without Local Search", LNS, path),
            ("Large Neighborhood Search with Local Search", LNS_LS, path_ls),
        ]:
            st.header(name)
            st.subheader("Pseudocode")
            st.markdown(code)
            st.subheader(ds)
            fig = tsp_plotter.plot(path)
            st.pyplot(fig)
            st.write(f"Best found path: {path}")

        df = pd.DataFrame({
            "name": ["LNS w/o LS Scores", "LNS with LS Scores", "LNS w/o LS Counts", "LNS with LS Counts"],
            "min": [min(scores), min(scores_ls), min(counts), min(counts_ls)],
            "mean": [mean(scores), mean(scores_ls), min(counts), mean(counts_ls)],
            "max": [max(scores), max(scores_ls), min(counts), max(counts_ls)],
        })
        st.header(ds)
        st.subheader("Scores")
        st.dataframe(df)

    st.divider()
    st.subheader("Conclusions")
    st.markdown(CONCLUSIONS)
    
    st.subheader("Comparisons")
    from pages.regret import ALGORITHMS as REGRET_ALGORITHMS
    from pages.local import ALGORITHMS as LOCAL_ALGORITHMS
    for ds in ["TSPA", "TSPB"]:
        st.subheader(ds)
        solution_data = load_algorithm_results([LOCAL_ALGORITHMS[5],REGRET_ALGORITHMS[1]], ds)
        st.dataframe(to_dataframe(solution_data))
    st.image("msls.png")
    
