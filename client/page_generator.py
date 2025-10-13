from dataclasses import dataclass

import plotly.express as px
import pandas as pd
import streamlit as st
import streamlit.components.v1 as components

import evolutionary
from utils import TSPPlotter, cache_to_disk


@st.cache_resource
def plot_animation(tsp_plotter, best_paths, algorithm):
    return tsp_plotter.plot_animated(best_paths[algorithm.work_name], algorithm.name)


@st.cache_resource
def plot_complexity(algorithms, state):
    data = {
        "size": list(range(2, 201)),
    }
    for algorithm in algorithms:
        data[algorithm.work_name] = evolutionary.complexity(
            state.replace(" ", ""), algorithm.work_name
        )
    return pd.DataFrame(data)


@dataclass
class Algorithm:
    name: str
    work_name: str
    pseudocode: str


def load_solution() -> tuple[pd.DataFrame, dict[str, float], dict[str, list[int]]]:
    """Loads solutions from the json

    Returns:
        DataFrame with results, times for each algorithm, best found solution for each algorithm
    """
    state = st.session_state.get("tsp_version")

    if not isinstance(state, str) and state not in ["TSP A", "TSP B"]:
        raise ValueError(f"Impossible TSP state reached: {state}")
    # algs = [alg for alg in ["random", "nn_to_last_point", "nn_to_any_point", "greedy_cycle"] if st.session_state.get(alg)]
    algs = ["random", "nn_to_last_point", "nn_to_any_point", "greedy_cycle"]
    solution_data = evolutionary.main(state.replace(" ", ""), algs)

    # proof that it works
    # print(evolutionary.complexity(state.replace(" ", ""),"greedy_cycle"))

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
    # for alg in [alg.work_name for alg in algorithms]:
    #     st.checkbox(alg,True, key=alg)

    df, times, best_paths = load_solution()
    col1, col2 = st.columns([1, 1])
    with col1:
        fig = px.box(df, labels={"variable": "", "value": "Cycle Cost"})
        st.plotly_chart(fig)

    with col2:
        fig = px.bar(
            pd.DataFrame({name: [time] for name, time in times.items()}).T,
            labels={"index": "", "value": "Processing time [s] (200 runs)"},
        )
        st.plotly_chart(fig)

    tabs = st.tabs([algorithm.name for algorithm in algorithms])

    state = st.session_state.get("tsp_version")
    if state not in ["TSP A", "TSP B"]:
        raise ValueError(f"Impossible TSP state reached: {state}")

    tsp_plotter = TSPPlotter(state)
    for algorithm, tab in zip(algorithms, tabs):
        with tab:
            animation = cache_to_disk(
                plot_animation,
                f"animation-{algorithm.name}-{state}",
                tsp_plotter,
                best_paths,
                algorithm,
            )
            components.html(animation.to_jshtml(), height=500)
            st.subheader("Pseudocode")
            st.markdown(algorithm.pseudocode)

    st.divider()
    if conclusions is not None:
        st.subheader("Conclusions")
        st.markdown(conclusions)

    df = cache_to_disk(plot_complexity, "complexity", algorithms, state)
    st.line_chart(df, x="size", y=[algorithm.work_name for algorithm in algorithms])

    st.subheader("Appendix")
    tabs = st.tabs([algorithm.name for algorithm in algorithms])
    for algorithm, tab in zip(algorithms, tabs):
        with tab:
            st.write("Best found path")
            st.write(best_paths[algorithm.work_name])
            st.write(f"Average value of algorithm is: {df[algorithm.work_name].mean()}")
