import pandas as pd
import numpy as np
import streamlit as st

from components.page_template import Algorithm
from components.tsp_plot import TSPPlotter
from problem import main
from utils import load_algorithm_results


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
        ["min score", "mean score", "max score", "min time [s]", "mean time [s]", "max time [s]", "total time [s]"],
    )


def report(algorithms: list[Algorithm], name: str, additional_algorithms: list[Algorithm] | None = None, conclusions: str | None = None):
    def load_solution(
        state,
    ) -> tuple[pd.DataFrame, dict[str, list[int]]]:
        """Loads solutions from the json

        Returns:
            DataFrame with results, times for each algorithm, best found solution for each algorithm
        """

        if not isinstance(state, str) and state not in ["TSP A", "TSP B"]:
            raise ValueError(f"Impossible TSP state reached: {state}")

        solution_data = load_algorithm_results(algorithms, state.replace(" ", ""))
        df = to_dataframe(solution_data)
        if additional_algorithms is not None:
            additional_solution_data = load_algorithm_results(additional_algorithms, state.replace(" ", ""))

            df = pd.concat([df, to_dataframe(additional_solution_data)])

        best_solutions = {
            solution.name: solution.best_solution for solution in solution_data
        }
        return df, best_solutions

    main(report=True)

    st.title(name)
    tsp_plotter_a = TSPPlotter("TSP A", dark_mode=False)  # type: ignore
    tsp_plotter_b = TSPPlotter("TSP B", dark_mode=False)  # type: ignore
    df_a, best_paths_a = load_solution("TSP A")
    df_b, best_paths_b = load_solution("TSP B")
    for algorithm in algorithms:
        st.header(algorithm.name)
        st.subheader("Pseudocode")
        st.markdown(algorithm.pseudocode)
        st.subheader("TSP A")
        fig = tsp_plotter_a.plot(best_paths_a[algorithm.work_name])
        st.pyplot(fig)
        st.write(f"Best found path: {best_paths_a[algorithm.work_name]}")
        st.subheader("TSP B")
        fig = tsp_plotter_b.plot(best_paths_b[algorithm.work_name])
        st.pyplot(fig)
        st.write(f"Best found path: {best_paths_b[algorithm.work_name]}")

    st.header("TSP A")
    st.subheader("Scores")
    row_height = 35
    height = row_height*(len(df_a)+1)
    st.dataframe(df_a[["min score", "mean score", "max score"]], height=height)
    st.subheader("Times")
    st.dataframe(df_a[["min time [s]", "mean time [s]", "max time [s]", "total time [s]"]], height=height)

    st.header("TSP B")
    st.subheader("Scores")
    height = row_height*(len(df_b)+1)
    st.dataframe(df_b[["min score", "mean score", "max score"]], height=height)
    st.subheader("Times")
    st.dataframe(df_b[["min time [s]", "mean time [s]", "max time [s]", "total time [s]"]], height=height)

    st.divider()
    if conclusions is not None:
        st.subheader("Conclusions")
        st.markdown(conclusions)
