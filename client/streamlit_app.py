import streamlit as st
import os
from textwrap import dedent

if st.session_state.get("report_mode"):
    st.markdown(
        dedent("""
    <style id="report-reset">
      /* Force white, neutral look in Report mode */
      html, body, [data-testid="stAppViewContainer"] {
        background: #ffffff !important;
        color: #111 !important;
        text-shadow: none !important;
        box-shadow: none !important;
        filter: none !important;
      }
      header[data-testid="stHeader"],
      [data-testid="stToolbar"] {
        background: transparent !important;
        box-shadow: none !important;
        backdrop-filter: none !important;
        border: none !important;
      }
      /* Inputs & buttons */
      .stButton > button,
      input, textarea, select,
      .stTextInput input, .stTextArea textarea, .stNumberInput input {
        background: #fff !important;
        color: #111 !important;
        border: 1px solid #ddd !important;
        text-shadow: none !important;
        box-shadow: none !important;
      }
      /* Tables & code blocks */
      pre, code, .stCodeBlock {
        background: #f6f8fa !important;
        color: #111 !important;
        border: 1px solid #e5e7eb !important;
        text-shadow: none !important;
      }
      [data-testid="stTable"] thead th {
        background: #f5f5f5 !important;
        color: #111 !important;
      }
      [data-testid="stTable"] tbody td {
        background: #ffffff !important;
        color: #111 !important;
      }
    </style>
    """),
        unsafe_allow_html=True,
    )
else:
    with open("vapor.css") as f:
        st.markdown(f"<style id='vapor-css'>{f.read()}</style>", unsafe_allow_html=True)

st.sidebar.selectbox("Select TSP version", ("TSP A", "TSP B"), key="tsp_version")
pages = [
    st.Page("problem.py", title="Problem description"),
    st.Page(os.path.join("pages", "greedy.py"), title="1. Greedy"),
    st.Page(os.path.join("pages", "regret.py"), title="2. Regret"),
]

pg = st.navigation(pages)
pg.run()
