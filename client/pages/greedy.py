import os
from typing import Any
import streamlit as st
import json
import plotly.express as px
import pandas as pd


def load_solution() -> tuple[pd.DataFrame, dict[str, float], dict[str, list[int]]]:
    """Loads solutions from the json

    Returns:
        DataFrame with results, times for each algorithm, best found solution for each algorithm
    """
    with open(os.path.join("data", "mock_data.json"), "r", encoding="utf-8") as f:
        solution_data: list[dict[str, Any]] = json.load(f)

    df = pd.DataFrame(
        {solution["name"]: solution["results"] for solution in solution_data}
    )

    times = {solution["name"]: solution["total_time"] for solution in solution_data}
    best_paths = {solution["name"]: solution["best_path"] for solution in solution_data}

    return df, times, best_paths


@st.cache_resource
def get_description():
    return {
        "Nearest Neighbor to the last point": "description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla, description bla bla bla"
    }


def main():
    st.title("Greedy algorithms")

    # st.subheader("text")
    # st.markdown("text")
    # st.divider()
    df, times, best_paths = load_solution()
    col1, col2 = st.columns([1, 1])
    with col1:
        fig = px.box(df)
        st.plotly_chart(fig)

    with col2:
        fig = px.bar(pd.DataFrame({name: [time] for name, time in times.items()}).T)
        st.plotly_chart(fig)

    st.divider()
    algorithm_full_name = st.selectbox(
        "Choose algorithm",
        [
            "Nearest Neighbor to the last point",
            "Nearest Neighbor to best point in cycle",
            "Greedy Cycle",
        ],
    )

    st.markdown(get_description()[algorithm_full_name])


if __name__ == "__main__":
    main()
