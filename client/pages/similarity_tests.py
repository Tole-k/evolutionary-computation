from utils import dill_cache
from problem import main
import streamlit as st
import evolutionary
import plotly.express as px
import pandas as pd

PSEUDOCODE = r"""
"""

CONCLUSIONS = r"""
"""


def generate_plot(tsp_version: str, measure: str):

    @dill_cache(f"similarity-{tsp_version}-{measure}")
    def get_data():
        return evolutionary.similarity_tests(tsp_version, measure)

    results, correlations = get_data()
    corr_best, corr_very_good, corr_avg = correlations

    data_x, data_y = list(zip(*results))
    data_best, data_very_good, data_avg = list(zip(*data_y))
    excluded = data_best.index(-1)
    tmp_data_x = list(data_x)
    tmp_data_best = list(data_best)
    tmp_data_best.pop(excluded)
    tmp_data_x.pop(excluded)

    col1, col2, col3 = st.columns(3)

    with col1:
        df_best = pd.DataFrame({"Cost": tmp_data_x, "Similarity": tmp_data_best})
        fig_best = px.scatter(df_best, x="Cost", y="Similarity", title=f"Similarity to best found solution (r={corr_best:.3f})")
        fig_best.update_layout(xaxis=dict(nticks=10), yaxis=dict(nticks=10))
        st.plotly_chart(fig_best)

    with col2:
        df_very_good = pd.DataFrame({"Cost": data_x, "Similarity": data_very_good})
        fig_very_good = px.scatter(
            df_very_good, x="Cost", y="Similarity", title=f"Similarity to a very good solution (ILS) (r={corr_very_good:.3f})"
        )
        fig_very_good.update_layout(xaxis=dict(nticks=10), yaxis=dict(nticks=10))
        st.plotly_chart(fig_very_good)

    with col3:
        df_avg = pd.DataFrame({"Cost": data_x, "Similarity": data_avg})
        fig_avg = px.scatter(df_avg, x="Cost", y="Similarity", title=f"Avg similarity to other found solutions (r={corr_avg:.3f})")
        fig_avg.update_layout(xaxis=dict(nticks=10), yaxis=dict(nticks=10))
        st.plotly_chart(fig_avg)


def report():
    main(report=True)
    for state in ["TSPA", "TSPB"]:
        st.header(f"Results for {state.replace('A', ' A').replace('B', ' B')}")
        for measure in ["node", "edge"]:
            st.subheader(f"{measure} similarity")
            generate_plot(state, measure)


def page():
    state = st.session_state.get("tsp_version")
    if state not in ["TSP A", "TSP B"]:
        raise ValueError(f"Impossible TSP state reached: {state}")
    measure = st.selectbox("Measure", ["node", "edge"])
    st.subheader(f"{measure} similarity")
    generate_plot(state.replace(" ", ""), measure)


if __name__ == "__main__":
    st.title("Multiple Start Local Search")
    st.set_page_config(layout="wide")
    if st.session_state.get("report_mode"):
        report()
    else:
        page()
