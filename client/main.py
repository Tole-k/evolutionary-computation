import streamlit as st
from utils import load_TSP_data

st.title("Evolutionary Computation lab")
st.header("Problem description")
st.markdown(
    "description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla"
)

st.header("Data")
state = st.session_state.get("tsp_version")

if state not in ["TSP A", "TSP B"]:
    raise ValueError(f"Impossible TSP state reached: {state}")

data = load_TSP_data(state)
st.dataframe(data)
