from collections.abc import Callable
from dataclasses import dataclass
from functools import wraps
import os
from typing import Literal
import hashlib
import base64

import streamlit as st
import pandas as pd
import dill


@dataclass
class Algorithm:
    name: str
    work_name: str
    pseudocode: str


def short_hash(s: str, length=10) -> str:
    h = hashlib.blake2b(s.encode(), digest_size=8).digest()
    return base64.urlsafe_b64encode(h).decode("ascii")[:length]


def cache_to_disk[**P, R](
    func: Callable[P, R],
    cache_filename: str,
    args: tuple | None = None,
    kwargs: dict | None = None,
) -> R:
    if args is None:
        args = tuple()
    if kwargs is None:
        kwargs = dict()
    cache_filename = short_hash(cache_filename) + ".dill"
    path = os.path.join(".custom_cache", cache_filename)
    if not os.path.exists(".custom_cache"):
        os.makedirs(".custom_cache")
    if os.path.exists(path):
        with open(path, "rb") as f:
            result = dill.load(f)
            print("loading")
    else:
        result = func(*args, **kwargs)
        with open(path, "wb") as f:
            dill.dump(result, f)
            print("dumping")
    return result


def dill_cache(filename: str):
    def _outer[**P, R](func: Callable[P, R]) -> Callable[P, R]:
        @wraps(func)
        def _inner_func(*args, **kwargs):
            return cache_to_disk(func, filename, args, kwargs)

        return _inner_func

    return _outer


@st.cache_data
def load_TSP_data(tsp_to_load: Literal["TSP A", "TSP B"]):
    tsp_path_map = {"TSP A": "TSPA.csv", "TSP B": "TSPB.csv"}

    instance = pd.read_csv(
        os.path.join("data", tsp_path_map[tsp_to_load]), sep=";", header=None
    )

    instance.columns = ["X coordinate", "Y coordinate", "cost"]
    return instance
