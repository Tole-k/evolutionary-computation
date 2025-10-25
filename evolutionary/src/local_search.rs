use core::f64;

use crate::greedy_algorithms::greedy_cycle;
use crate::local_search_base::local_search;
use crate::utils::{DataPoint, generate_random_solution};
use ndarray::Array2;

pub fn ls_greedy_edges_random(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, true)
}

pub fn ls_greedy_edges_greedy(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = greedy_cycle(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, true)
}

pub fn ls_greedy_nodes_random(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, true)
}

pub fn ls_greedy_nodes_greedy(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = greedy_cycle(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, true)
}

pub fn ls_steepest_edges_random(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, false)
}

pub fn ls_steepest_edges_greedy(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = greedy_cycle(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, false)
}

pub fn ls_steepest_nodes_random(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, false)
}

pub fn ls_steepest_nodes_greedy(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = greedy_cycle(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, false)
}
