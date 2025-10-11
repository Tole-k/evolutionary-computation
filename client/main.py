import streamlit as st
from data_loader import load_TSP_data_A

st.title("Evolutionary Computation lab")
st.header("Problem description")
st.markdown(
    "description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla"
)
# st.subheader
# st.divider
st.header("Data")
data = load_TSP_data_A()
st.dataframe(data)
