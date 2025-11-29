from statistics import mean
from utils import Algorithm
import pandas as pd
import numpy as np
import streamlit as st

from components.tsp_plot import TSPPlotter
from problem import main


LNS = r"""
"""

CONCLUSIONS = r"""
"""

ALGORITHMS = [
    Algorithm(
        "Large Neighborhood Search with Local Search",
        "large_neighborhood_search_w_ls",
        LNS,
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
    tsp_plotter_a = TSPPlotter("TSP A", dark_mode=False)  # type: ignore
    tsp_plotter_b = TSPPlotter("TSP B", dark_mode=False)  # type: ignore
    import evolutionary

    counts_a, scores_a, path_a, counts_a_ls,scores_a_ls, path_a_ls = evolutionary.assignment_7("TSPA", 0.3)
    counts_b, scores_b, path_b, counts_b_ls, scores_b_ls, path_b_ls = evolutionary.assignment_7("TSPB", 0.3)
    for name, code, path_a, path_b in [
        ("Large Neighborhood Search with Local Search", LNS, path_a, path_b),
        ("Large Neighborhood Search without Local Search", LNS, path_a_ls, path_b_ls),
    ]:
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

    df_a = pd.DataFrame({
        "name": ["LNS w/o LS Scores", "LNS with LS Scores", "LNS w/o LS Counts", "LNS with LS Counts"],
        "min": [min(scores_a), min(scores_a_ls), min(counts_a), min(counts_a_ls)],
        "mean": [mean(scores_a), mean(scores_a_ls), min(counts_a), mean(counts_a_ls)],
        "max": [max(scores_a), max(scores_a_ls), min(counts_a), max(counts_a_ls)],
    })
    st.header("TSP A")
    st.subheader("Scores")
    st.dataframe(df_a)

    df_b = pd.DataFrame({
        "name": ["LNS w/o LS Scores", "LNS with LS Scores", "LNS w/o LS Counts", "LNS with LS Counts"],
        "min": [min(scores_b), min(scores_b_ls), min(counts_b), min(counts_b_ls)],
        "mean": [mean(scores_b), mean(scores_b_ls), min(counts_b), mean(counts_b_ls)],
        "max": [max(scores_b), max(scores_b_ls), min(counts_b), max(counts_b_ls)],
    })
    st.header("TSP B")
    st.subheader("Scores")
    st.dataframe(df_b)

    st.divider()
    st.subheader("Conclusions")
    st.markdown(CONCLUSIONS)
