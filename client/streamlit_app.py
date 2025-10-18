import streamlit as st
import os

st.sidebar.selectbox("Select TSP version", ("TSP A", "TSP B"), key="tsp_version")
st.sidebar.markdown(
    "[Link to source code](https://github.com/tole-k/evolutionary-computation)"
)
pages = [
    st.Page("problem.py", title="Problem description"),
    st.Page(os.path.join("pages", "greedy.py"), title="1. Greedy"),
    st.Page(os.path.join("pages", "regret.py"), title="2. Regret"),
]

pg = st.navigation(pages)
pg.run()
