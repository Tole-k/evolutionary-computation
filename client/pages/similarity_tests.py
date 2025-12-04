from matplotlib import pyplot as plt
from matplotlib.axes import Axes
from problem import main
import streamlit as st
import evolutionary

PSEUDOCODE = r"""
"""

CONCLUSIONS = r"""
"""


def present_results(tsp_version: str, measure: str):
    st.subheader(f"{measure} similarity")
    data_x, data_y = list(zip(*evolutionary.similarity_tests(tsp_version, measure)))
    data_best, data_very_good, data_avg = list(zip(*data_y))
    ax: list[Axes]
    fig, ax = plt.subplots(ncols=3)
    excluded = data_best.index(-1)
    tmp_data_x = list(data_x)
    tmp_data_best = list(data_best)
    tmp_data_best.pop(excluded)
    tmp_data_x.pop(excluded)
    ax[0].scatter(tmp_data_x, tmp_data_best)
    ax[0].title.set_text("Similarity to best found solution")
    ax[1].scatter(data_x, data_very_good)
    ax[1].title.set_text("Similarity to a very good solution (ILS)")
    ax[2].scatter(data_x, data_avg)
    ax[2].title.set_text("Avg similarity to other found solutions")
    st.pyplot(fig)


def report():
    main(report=True)
    for state in ["TSPA", "TSPB"]:
        for measure in ["node", "edge"]:
            present_results(state, measure)


def page():
    state = st.session_state.get("tsp_version")
    if state not in ["TSP A", "TSP B"]:
        raise ValueError(f"Impossible TSP state reached: {state}")
    measure = st.selectbox("Measure", ["node", "edge"])
    present_results(state.replace(" ", ""), measure)


if __name__ == "__main__":
    st.title("Multiple Start Local Search")
    if st.session_state.get("report_mode"):
        report()
    else:
        page()
