from typing import Literal
from collections.abc import Iterable
from dataclasses import dataclass

import matplotlib
from matplotlib.figure import Figure
from matplotlib.axes import Axes
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
from matplotlib.collections import LineCollection

from utils import load_TSP_data

matplotlib.rcParams["animation.embed_limit"] = 2**128

STREAMLIT_DARK = "#0E1117"
WHITE = "#F3F3F3"


@dataclass(frozen=True)
class EdgeStyle:
    tour_color: str = "#59FB5EA1"
    delta_color: str = "#FF4D4D"
    tour_lw: float = 2.0
    delta_lw: float = 3.0


class TSPPlotter:
    def __init__(
        self, tsp_to_load: Literal["TSP A", "TSP B"], dark_mode: bool = True
    ) -> None:
        instance = load_TSP_data(tsp_to_load).T.to_numpy()
        self.tsp_to_load = tsp_to_load
        self.x_coords, self.y_coords, self.costs = instance
        self.dark_mode: bool = dark_mode
        self._style = EdgeStyle()

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
        # Draw full, static tour
        for idx1, idx2 in zip(solution, solution[1:] + [solution[0]]):
            ax.plot(
                [self.x_coords[idx1], self.x_coords[idx2]],
                [self.y_coords[idx1], self.y_coords[idx2]],
                "o--",
                linewidth=2,
                color=self._style.tour_color,
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
        # keeps your old node-by-node animation
        fig, ax = self.prepare_fig()
        new_solution = [solution[0]]

        def update(i):
            idx1, idx2 = (new_solution[-1], solution[i - len(solution) + 1])
            ax.plot(
                [self.x_coords[idx1], self.x_coords[idx2]],
                [self.y_coords[idx1], self.y_coords[idx2]],
                "o--",
                linewidth=2,
                color=self._style.tour_color,
            )
            new_solution.append(solution[i - len(solution) + 1])

        animation = FuncAnimation(fig, update, len(solution))  # type: ignore
        return animation

    def plot_evolution(
        self,
        solutions: list[list[int]],
        interval_ms: int = 600,
        directed: bool = False,
        blit: bool = True,
        repeat: bool = False,
    ) -> FuncAnimation:
        """
        Animate a sequence of full tours.
        - Shows the current tour in green.
        - Overlays, in red, only edges that changed compared to the previous frame.
        """
        if not solutions:
            raise ValueError("solutions must be a non-empty list of tours")

        fig, ax = self.prepare_fig()

        tour_lc = LineCollection(
            [], linewidths=self._style.tour_lw, colors=[self._style.tour_color]
        )
        delta_lc = LineCollection(
            [], linewidths=self._style.delta_lw, colors=[self._style.delta_color]
        )

        ax.add_collection(tour_lc)
        ax.add_collection(delta_lc)

        edge_sets = [self._path_to_edges(t, directed=directed) for t in solutions]

        def set_segments_from_edges(
            lc: LineCollection, edges: Iterable[tuple[int, int]]
        ):
            segs = [
                (
                    [self.x_coords[i], self.y_coords[i]],
                    [self.x_coords[j], self.y_coords[j]],
                )
                for i, j in edges
            ]
            lc.set_segments(segs)

        def update(frame_idx: int):
            curr_edges = edge_sets[frame_idx]
            set_segments_from_edges(tour_lc, curr_edges)

            if frame_idx == 0:
                delta_lc.set_segments([])
            else:
                prev_edges = edge_sets[frame_idx - 1]
                changed = curr_edges ^ prev_edges
                set_segments_from_edges(delta_lc, changed)

            return (tour_lc, delta_lc)

        anim = FuncAnimation(
            fig,
            update,
            frames=len(solutions),
            interval=interval_ms,
            blit=blit,
            repeat=repeat,
        )
        return anim

    def _path_to_edges(
        self, path: list[int], directed: bool = False
    ) -> "set[tuple[int, int]]":
        """
        Convert cyclic tour path into a set of edges.
        If directed=False, treat edges as undirected by sorting endpoints.
        """
        if len(path) < 2:
            return set()
        pairs = list(zip(path, path[1:] + [path[0]]))
        if directed:
            return set(pairs)
        # undirected: normalize so (i,j)==(j,i)
        return {tuple(sorted(p)) for p in pairs}
