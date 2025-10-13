import streamlit as st
from utils import load_TSP_data

DESCRIPTION = """
Problem that we are using to benchmark different algorithms is modified version of **TSP**.

In this problem each nodes are placed in euclidean space, with additional cost for visiting each one of them.

Our goal is to visit half (rounded up) nodes, and go back to the start is such way that the travel cost is minimized (calculating euclidean distance and with added cost)
"""

st.title("Evolutionary Computation lab")
st.header("Problem description")
st.markdown(DESCRIPTION)

st.header("Data")
state = st.session_state.get("tsp_version")

if state not in ["TSP A", "TSP B"]:
    raise ValueError(f"Impossible TSP state reached: {state}")

data = load_TSP_data(state)
st.dataframe(data)
