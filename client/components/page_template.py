import pandas as pd
import streamlit as st

import evolutionary
from utils import Algorithm
from components.simple_plots import comparison_plots, plot_complexity
from components.algorithm_explanation import algorithms_tabs


def plot_animation(tsp_plotter, best_paths, algorithm):
    return tsp_plotter.plot_animated(
        best_paths[algorithm.work_name], algorithm.name
    ).to_jshtml(default_mode="once")


def load_solution(algorithms: list[Algorithm]) -> tuple[pd.DataFrame, dict[str, float], dict[str, list[int]]]:
    """Loads solutions from the json

    Returns:
        DataFrame with results, times for each algorithm, best found solution for each algorithm
    """
    state = st.session_state.get("tsp_version")

    if not isinstance(state, str) and state not in ["TSP A", "TSP B"]:
        raise ValueError(f"Impossible TSP state reached: {state}")
    solution_data = evolutionary.main(state.replace(" ", ""), [alg.work_name for alg in algorithms])  # type: ignore

    df = pd.DataFrame({solution.name: solution.scores for solution in solution_data})

    times = {solution.name: solution.total_time for solution in solution_data}
    best_solutions = {
        solution.name: solution.best_solution for solution in solution_data
    }

    return df, times, best_solutions


def algorithm_comparison_page(
    algorithms: list[Algorithm], name: str, conclusions: str | None = None
):
    st.title(name)

    df, times, best_paths = load_solution(algorithms)
    comparison_plots(df, times)

    algorithms_tabs(algorithms, best_paths)

    st.divider()
    if conclusions is not None:
        st.subheader("Conclusions")
        st.markdown(conclusions)

    state = st.session_state.get("tsp_version")
    if state not in ["TSP A", "TSP B"]:
        raise ValueError(f"Impossible TSP state reached: {state}")

    plot_complexity(algorithms, state)

    st.subheader("Appendix")
    tabs = st.tabs([algorithm.name for algorithm in algorithms])
    for algorithm, tab in zip(algorithms, tabs):
        with tab:
            st.write("Best found path")
            st.write(best_paths[algorithm.work_name])
            st.write(f"Average value of algorithm is: {df[algorithm.work_name].mean()}")
