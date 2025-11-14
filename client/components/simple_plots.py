import plotly.express as px
import pandas as pd
import streamlit as st
import evolutionary
from utils import Algorithm, dill_cache


def comparison_plots(df, times):
    col1, col2 = st.columns([1, 1])
    with col1:
        fig = px.box(df, labels={"variable": "", "value": "Cycle Cost"})
        fig.update_layout(
            paper_bgcolor="rgba(0,0,0,0)",
            plot_bgcolor="rgba(0,0,0,0)",
            font_color="#e6f7ff",
        )
        st.plotly_chart(fig)

    with col2:
        fig = px.bar(
            pd.DataFrame({name: [time] for name, time in times.items()}).T,
            labels={"index": "", "value": "Processing time [s]"},
        )
        fig.update_layout(
            paper_bgcolor="rgba(0,0,0,0)",
            plot_bgcolor="rgba(0,0,0,0)",
            font_color="#e6f7ff",
        )
        st.plotly_chart(fig)


def _plot_complexity_worker(algorithms: list[Algorithm], state):
    data: dict[str, list[int | float]] = {
        "size": list(range(2, 201)),
    }
    for algorithm in algorithms:
        data[algorithm.work_name] = evolutionary.complexity(
            state.replace(" ", ""), algorithm.work_name
        )
    return pd.DataFrame(data)


def plot_complexity(algorithms: list[Algorithm], state: str):
    hash_name = (
        "complexity-" + str([algorithm.name for algorithm in algorithms]) + state
    )
    time_df = dill_cache(hash_name)(_plot_complexity_worker)(algorithms, state)
    st.line_chart(
        time_df, x="size", y=[algorithm.work_name for algorithm in algorithms]
    )
