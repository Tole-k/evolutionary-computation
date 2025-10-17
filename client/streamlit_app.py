import streamlit as st
import os

with open("vapor.css") as f:
    st.markdown(f"<style>{f.read()}</style>", unsafe_allow_html=True)

st.sidebar.selectbox("Select TSP version", ("TSP A", "TSP B"), key="tsp_version")
pages = [
    st.Page("problem.py", title="Problem description"),
    st.Page(os.path.join("pages", "greedy.py"), title="1. Greedy"),
    st.Page(os.path.join("pages", "regret.py"), title="2. Regret"),
]

pg = st.navigation(pages)
pg.run()
