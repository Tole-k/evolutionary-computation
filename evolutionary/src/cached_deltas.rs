use ndarray::Array2;

use crate::cached_deltas_base::{local_search_w_cached_deltas, local_search_w_cached_deltas_full};
use crate::utils::DataPoint;

pub fn ls_cached_deltas_edges(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    local_search_w_cached_deltas(data, starting_point_index, distance_matrix, true)
}

pub fn ls_cached_deltas_nodes(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    local_search_w_cached_deltas(data, starting_point_index, distance_matrix, false)
}
pub fn ls_cached_deltas_edges_full(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<Vec<usize>> {
    local_search_w_cached_deltas_full(data, starting_point_index, distance_matrix, true)
}

pub fn ls_cached_deltas_nodes_full(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<Vec<usize>> {
    local_search_w_cached_deltas_full(data, starting_point_index, distance_matrix, false)
}
