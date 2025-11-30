import json
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
            lns, lns_w_ls = evolutionary.assignment_7(ds, 0.3)
            _, (counts, scores, path) = lns
            _, (counts_ls, scores_ls, path_ls) = lns_w_ls
            print("saved results not found calculating new ones")
        
        for name, code, path in [
            ("Large Neighborhood Search without Local Search", LNS, path),
            ("Large Neighborhood Search with Local Search", LNS, path_ls),
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
