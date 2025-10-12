import streamlit as st
import os

st.sidebar.selectbox("Select TSP version", ("TSP A", "TSP B"), key="tsp_version")

pages = [
    st.Page("main.py", title="Problem description"),
    st.Page(os.path.join("pages", "greedy.py"), title="1. Greedy"),
]

pg = st.navigation(pages)
pg.run()
