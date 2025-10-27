from components.tsp_plot import TSPPlotter
import evolutionary
import streamlit.components.v1 as components
import streamlit as st
import random

algorithm = st.selectbox(
    "Algorithms",
    options=[
        "ls_greedy_edges_random",
        "ls_greedy_edges_greedy",
        "ls_greedy_nodes_random",
        "ls_greedy_nodes_greedy",
        "ls_steepest_edges_random",
        "ls_steepest_edges_greedy",
        "ls_steepest_nodes_random",
        "ls_steepest_nodes_greedy",
    ],
)

animation = (
    TSPPlotter("TSP A")
    .plot_evolution(
        evolutionary.solution_history(
            st.session_state.get("tsp_version").replace(" ", ""),
            algorithm,
            random.randint(0, 199),
        )
    )
    .to_jshtml(5)
)
components.html(animation, height=500)

# algorithm_comparison_page(ALGORITHMS, "test")
