import plotly.express as px
import pandas as pd
import streamlit as st
import streamlit.components.v1 as components

import evolutionary
from client.components.page_template import Algorithm
from utils import TSPPlotter
from problem import main


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
        solution_data = evolutionary.main(state.replace(" ", ""), [alg.work_name for alg in algorithms])

        df = pd.DataFrame(
            {solution.name: solution.scores for solution in solution_data}
        )

        times = {solution.name: solution.total_time for solution in solution_data}
        best_solutions = {
            solution.name: solution.best_solution for solution in solution_data
        }

        return df, times, best_solutions

    main(load_data=False)
    st.title(name)
    for state in ["TSP A", "TSP B"]:
        st.header(state)
        df, times, best_paths = load_solution(state)
        col1, col2 = st.columns([1, 1])
        with col1:
            fig = px.box(df)
            st.plotly_chart(fig)

        with col2:
            fig = px.bar(pd.DataFrame({name: [time] for name, time in times.items()}).T)
            st.plotly_chart(fig)
    for algorithm in algorithms:
        st.header(algorithm.name)
        for state in ["TSP A", "TSP B"]:
            st.header(state)
            df, times, best_paths = load_solution(state)
            tsp_plotter = TSPPlotter(state)
            animation = tsp_plotter.plot_animated(
                best_paths[algorithm.work_name], algorithm.name
            )
            components.html(animation.to_jshtml(default_mode="once"), height=500)
        st.subheader("Pseudocode")
        st.markdown(algorithm.pseudocode)

    st.divider()
    if conclusions is not None:
        st.subheader("Conclusions")
        st.markdown(conclusions)

    st.subheader("Appendix")
    st.write(
        "Source code is available on https://github.com/tole-k/evolutionary-computation.git"
    )
    for algorithm in algorithms:
        for state in ["TSP A", "TSP B"]:
            st.header(state)
            df, times, best_paths = load_solution(state)
            st.write("Best found path")
            st.write(str(best_paths[algorithm.work_name]))
            st.write(f"Min value of algorithm is: {df[algorithm.work_name].min()}")
            st.write(f"Average value of algorithm is: {df[algorithm.work_name].mean()}")
            st.write(f"Max value of algorithm is: {df[algorithm.work_name].max()}")
