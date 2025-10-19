from typing import Literal

import matplotlib
from matplotlib.figure import Figure
from matplotlib.axes import Axes
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

from utils import load_TSP_data

matplotlib.rcParams["animation.embed_limit"] = 2**128

STREAMLIT_DARK = "#0E1117"
WHITE = "#F3F3F3"


class TSPPlotter:
    def __init__(
        self, tsp_to_load: Literal["TSP A", "TSP B"], dark_mode: bool = True
    ) -> None:
        instance = load_TSP_data(tsp_to_load).T.to_numpy()
        self.x_coords, self.y_coords, self.costs = instance
        self.dark_mode: bool = dark_mode

    def plot(self, solution: list[int]) -> Figure:
        fig, ax = self.prepare_fig()
        self._apply_lines(ax, solution)

        return fig

    def prepare_fig(self) -> tuple[Figure, Axes]:
        fig = plt.figure(
            figsize=(8, 5),
            dpi=80,
            clear=True,
            edgecolor=WHITE if self.dark_mode else STREAMLIT_DARK,
            linewidth=2,
        )
        fig.patch.set_facecolor(STREAMLIT_DARK if self.dark_mode else WHITE)

        ax = fig.add_axes(
            (0, 0, 1, 1),
            frameon=False,
            xticks=[],
            yticks=[],
        )
        ax.set_facecolor(STREAMLIT_DARK if self.dark_mode else WHITE)
        self._scatter_plot_tsp(ax)

        return fig, ax

    def _apply_lines(self, ax: Axes, solution: list[int]):
        ax.scatter(self.x_coords, self.y_coords, s=self.costs / 10)
        for idx1, idx2 in zip(solution[:-1], solution[1:]):
            ax.plot(
                [self.x_coords[idx1], self.x_coords[idx2]],
                [self.y_coords[idx1], self.y_coords[idx2]],
                "o--",
                linewidth=2,
                color="#59FB5EA1",  # "#A31235F1",
            )
        ax.plot(
            [self.x_coords[solution[-1]], self.x_coords[solution[0]]],
            [self.y_coords[solution[-1]], self.y_coords[solution[0]]],
            "o--",
            linewidth=2,
            color="#59FB5EA1",  # "#A31235F1",
        )

    def _scatter_plot_tsp(self, ax: Axes):
        ax.scatter(
            self.x_coords,
            self.y_coords,
            s=self.costs / 10,
            c="#83C9FFAA",
            edgecolors="#83C9FF",
        )

    def plot_animated(self, solution: list[int]) -> FuncAnimation:
        fig, ax = self.prepare_fig()

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

        animation = FuncAnimation(fig, update, len(solution))  # type: ignore

        return animation
