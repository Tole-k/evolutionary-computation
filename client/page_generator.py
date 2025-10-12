from dataclasses import dataclass
import os

import plotly.express as px
import pandas as pd
import streamlit as st
import streamlit.components.v1 as components

import evolutionary
from utils import TSPPlotter


@dataclass
class Algorithm:
    name: str
    work_name: str
    description: str


def load_solution() -> tuple[pd.DataFrame, dict[str, float], dict[str, list[int]]]:
    """Loads solutions from the json

    Returns:
        DataFrame with results, times for each algorithm, best found solution for each algorithm
    """
    solution_data = evolutionary.main()

    df = pd.DataFrame({solution.name: solution.scores for solution in solution_data})

    times = {solution.name: solution.total_time for solution in solution_data}
    best_solutions = {
        solution.name: solution.best_solution for solution in solution_data
    }

    return df, times, best_solutions


def algorithm_comparison_page(algorithms: list[Algorithm], name: str):
    st.title(name)

    df, times, best_paths = load_solution()
    col1, col2 = st.columns([1, 1])
    with col1:
        fig = px.box(df)
        st.plotly_chart(fig)

    with col2:
        fig = px.bar(pd.DataFrame({name: [time] for name, time in times.items()}).T)
        st.plotly_chart(fig)

    st.divider()
    tabs = st.tabs([algorithm.name for algorithm in algorithms])
    tsp_plotter = TSPPlotter(os.path.join("data", "TSPA.csv"))
    for algorithm, tab in zip(algorithms, tabs):
        with tab:
            st.markdown(algorithm.description)
            animation = tsp_plotter.plot_animated(
                best_paths[algorithm.work_name], algorithm.name
            )
            components.html(animation.to_jshtml(), height=500)

    st.divider()
    st.subheader("Conclusions")
    # TODO: Conclusions
    st.markdown("")
