import streamlit as st
import os

st.sidebar.selectbox("Select TSP version", ("TSP A", "TSP B"), key="tsp_version")
st.sidebar.markdown(
    "[Link to source code](https://github.com/tole-k/evolutionary-computing)"
)
pages = [
    st.Page("problem.py", title="Problem description"),
    st.Page(os.path.join("pages", "greedy.py"), title="1. Greedy"),
    st.Page(os.path.join("pages", "report_like.py"), title="1. report"),
]

pg = st.navigation(pages)
pg.run()
