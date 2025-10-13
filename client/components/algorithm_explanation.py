import streamlit as st
import streamlit.components.v1 as components
from utils import Algorithm, cache_to_disk
from components.tsp_plot import TSPPlotter


def _plot_animation(
    tsp_plotter: TSPPlotter, best_path: list[int], algorithm: Algorithm
):
    return tsp_plotter.plot_animated(best_path, algorithm.name).to_jshtml(
        default_mode="once"
    )


def algorithm_page(
    algorithm: Algorithm,
    best_path: list[int],
    tsp_plotter: TSPPlotter,
    state: str | None = None,
):
    animation = cache_to_disk(
        _plot_animation,
        f"animation-{algorithm.name}-{state}",
        args=(tsp_plotter, best_path, algorithm),
    )
    components.html(animation, height=500)
    if len(algorithm.pseudocode) >= 5:
        st.subheader("Pseudocode")
        st.markdown(algorithm.pseudocode)


def algorithms_tabs(algorithms: list[Algorithm], best_paths: dict[str, list[int]]):
    state = st.session_state.get("tsp_version")
    if state not in ["TSP A", "TSP B"]:
        raise ValueError(f"Impossible TSP state reached: {state}")

    tsp_plotter = TSPPlotter(state)
    tabs = st.tabs([algorithm.name for algorithm in algorithms])
    for algorithm, tab in zip(algorithms, tabs):
        with tab:
            algorithm_page(
                algorithm, best_paths[algorithm.work_name], tsp_plotter, state
            )
