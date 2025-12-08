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
        "ls_candidate_10",
        "ls_candidate_25",
        "ls_candidate_50",
    ],
)


tsp_version: str = st.session_state.get("tsp_version", "TSP A")
history = evolutionary.solution_history(
    tsp_version.replace(" ", ""),
    algorithm,
    random.randint(0, 199),
)
for route in history:
    assert len(route) == 100
st.subheader(f"Steps: {len(history)}")
animation = TSPPlotter("TSP A").plot_evolution(history).to_jshtml(5)
components.html(animation, height=500)
