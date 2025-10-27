import pandas as pd
import streamlit as st

import evolutionary
from components.page_template import Algorithm
from components.tsp_plot import TSPPlotter
from problem import main


def _table(df: pd.DataFrame, algorithms: list[Algorithm]):
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


def report(algorithms: list[Algorithm], name: str, additional_algorithms: list[Algorithm] | None = None, conclusions: str | None = None):
    def load_solution(
        state,
    ) -> tuple[pd.DataFrame, dict[str, float], dict[str, list[int]]]:
        """Loads solutions from the json

        Returns:
            DataFrame with results, times for each algorithm, best found solution for each algorithm
        """

        if not isinstance(state, str) and state not in ["TSP A", "TSP B"]:
            raise ValueError(f"Impossible TSP state reached: {state}")

        solution_data = evolutionary.main_mc(
            state.replace(" ", ""), [alg.work_name for alg in algorithms]
        )
        df = pd.DataFrame(
            {solution.name: solution.scores for solution in solution_data}
        )
        if additional_algorithms is not None:
            additional_solution_data = evolutionary.main_mc(
                state.replace(" ", ""), [alg.work_name for alg in additional_algorithms]
            )

            df = pd.concat([df, pd.DataFrame({solution.name: solution.scores for solution in additional_solution_data})])

        times = {solution.name: solution.total_time for solution in solution_data}
        best_solutions = {
            solution.name: solution.best_solution for solution in solution_data
        }
        return df, times, best_solutions

    main(report=True)

    st.title(name)
    tsp_plotter_a = TSPPlotter("TSP A", dark_mode=False)  # type: ignore
    tsp_plotter_b = TSPPlotter("TSP B", dark_mode=False)  # type: ignore
    df_a, _, best_paths_a = load_solution("TSP A")
    df_b, _, best_paths_b = load_solution("TSP B")
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
    combined_algorithms = algorithms+additional_algorithms if additional_algorithms is not None else algorithms
    _table(df_a, combined_algorithms)
    st.header("TSP B")
    _table(df_b, combined_algorithms)
    st.divider()
    if conclusions is not None:
        st.subheader("Conclusions")
        st.markdown(conclusions)
