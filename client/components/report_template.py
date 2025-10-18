import pandas as pd
import streamlit as st
import streamlit.components.v1 as components

import evolutionary
from components.page_template import Algorithm
from components.tsp_plot import TSPPlotter
from problem import main
from components.algorithm_explanation import algorithm_page


def report(algorithms: list[Algorithm], name: str, conclusions: str | None = None):
    name = "Greedy algorithms"

    def load_solution(
        state,
    ) -> tuple[pd.DataFrame, dict[str, float], dict[str, list[int]]]:
        """Loads solutions from the json

        Returns:
            DataFrame with results, times for each algorithm, best found solution for each algorithm
        """

        if not isinstance(state, str) and state not in ["TSP A", "TSP B"]:
            raise ValueError(f"Impossible TSP state reached: {state}")

        solution_data = evolutionary.main(
            state.replace(" ", ""), [alg.work_name for alg in algorithms]
        )

        df = pd.DataFrame(
            {solution.name: solution.scores for solution in solution_data}
        )

        times = {solution.name: solution.total_time for solution in solution_data}
        best_solutions = {
            solution.name: solution.best_solution for solution in solution_data
        }

        return df, times, best_solutions

    main(report=True)

    st.title(name)

    for algorithm in algorithms:
        st.header(algorithm.name)
        for state in ["TSP A", "TSP B"]:
            st.header(state)
            _, _, best_paths = load_solution(state)
            tsp_plotter = TSPPlotter(state)
            algorithm_page(algorithm, best_paths[algorithm.work_name], tsp_plotter)
            animation = tsp_plotter.plot_animated(
                best_paths[algorithm.work_name], algorithm.name
            )
            components.html(animation.to_jshtml(default_mode="once"), height=500)
            st.write(f"Best found path: {best_paths[algorithm.work_name]}")
        st.subheader("Pseudocode")
        st.markdown(algorithm.pseudocode)

    st.divider()
    if conclusions is not None:
        st.subheader("Conclusions")
        st.markdown(conclusions)

    st.write(
        "Source code is available on https://github.com/tole-k/evolutionary-computation.git"
    )
    for algorithm in algorithms:
        for state in ["TSP A", "TSP B"]:
            st.header(state)
            df, _, _ = load_solution(state)
            new_df = pd.DataFrame(
                [
                    [
                        df[algorithm.work_name].min(),
                        df[algorithm.work_name].mean(),
                        df[algorithm.work_name].max(),
                    ]
                    for algorithm in algorithms
                ],
                [algorithm.name for algorithm in algorithms],
                ["min", "mean", "max"],
            )
            st.dataframe(new_df)
