from collections.abc import Iterable
import os
from typing import Literal

import matplotlib
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

import streamlit as st
import pandas as pd

matplotlib.rcParams["animation.embed_limit"] = 2**128


@st.cache_data
def load_TSP_data(tsp_to_load: Literal["TSP A", "TSP B"]):
    tsp_path_map = {"TSP A": "TSPA.csv", "TSP B": "TSPB.csv"}

    instance = pd.read_csv(
        os.path.join("data", tsp_path_map[tsp_to_load]), sep=";", header=None
    )

    instance.columns = ["X coordinate", "Y coordinate", "cost"]
    return instance


class TSPPlotter:
    def __init__(self, tsp_to_load: Literal["TSP A", "TSP B"]) -> None:
        instance = load_TSP_data(tsp_to_load).T.to_numpy()
        self.x_coords, self.y_coords, self.costs = instance

    def plot(self, instance: dict[str, list[int]]):
        return self._plot(instance)

    def plot_from_file(self, solution_path: str):
        if os.path.isfile(solution_path):
            solutions = {solution_path.removesuffix(".txt"): solution_path}
        else:
            solutions = {
                file_name.removesuffix(".txt"): os.path.join(solution_path, file_name)
                for file_name in os.listdir(solution_path)
                if file_name.endswith(".txt")
            }
        return solutions

    def _plot(self, solutions, early_stop: int | None = None):
        fig, axs = plt.subplots(1, len(solutions), figsize=(15, 5), dpi=150)

        if not isinstance(axs, Iterable):
            axs = [axs]
        for ax, (solution_name, solution) in zip(axs, solutions.items()):
            solution.append(solution[0])
            self.scatter_plot_tsp(ax, solution, solution_name, early_stop)

        return fig

    def scatter_plot_tsp(
        self, ax, solution: list[int], solution_name: str, early_stop: int | None = None
    ):
        ax.scatter(self.x_coords, self.y_coords, s=self.costs / 10)
        for id, (idx1, idx2) in enumerate(zip(solution[:-1], solution[1:])):
            ax.plot(
                [self.x_coords[idx1], self.x_coords[idx2]],
                [self.y_coords[idx1], self.y_coords[idx2]],
                color="red",
            )
            if id == early_stop:
                break

        ax.set_title(solution_name)
        ax.label_outer()

    def _scatter_plot_tsp(self, ax: plt.Axes, solution_name: str):
        ax.scatter(
            self.x_coords,
            self.y_coords,
            s=self.costs / 10,
            c="#83C9FFAA",
            edgecolors="#83C9FF",
        )

    def plot_animated(self, solution: list[int], solution_name: str) -> FuncAnimation:
        streamlit_color = "#0E1117"
        nice_whitey = "#FFDDFF"
        fig = plt.figure(
            figsize=(8, 5),
            dpi=80,
            clear=True,
            edgecolor=nice_whitey,
            linewidth=2,
        )

        fig.patch.set_facecolor(streamlit_color)

        ax = fig.add_axes(
            (0, 0, 1, 1),
            frameon=False,
            xticks=[],
            yticks=[],
        )
        ax.set_facecolor(streamlit_color)

        self._scatter_plot_tsp(ax, solution_name)

        new_solution = []
        new_solution.append(solution[0])

        def update(i):
            idx1, idx2 = (new_solution[-1], solution[i - len(solution) + 1])
            ax.plot(
                [self.x_coords[idx1], self.x_coords[idx2]],
                [self.y_coords[idx1], self.y_coords[idx2]],
                "o--",
                linewidth=2,
                color="#59FB5EA1",  # "#A31235F1",
            )
            new_solution.append(solution[i - len(solution) + 1])

        animation = FuncAnimation(fig, update, len(solution))

        return animation
