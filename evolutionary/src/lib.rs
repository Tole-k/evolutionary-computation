mod greedy_algorithms;
mod local_search;
mod local_search_base;
mod local_search_candidates;
mod multi_local_search;
mod regret_heuristics;
mod utils;
use ndarray::Array2;
use pyo3::prelude::*;
use std::{collections::HashMap, time::Instant, usize};

use crate::utils::{Metrics, check_solution};

fn get_map() -> HashMap<&'static str, fn(&Vec<utils::DataPoint>, usize, &Array2<f64>) -> Vec<usize>>
{
    HashMap::from([
        (
            "random",
            utils::generate_random_solution
                as fn(&Vec<utils::DataPoint>, usize, &Array2<f64>) -> Vec<usize>,
        ),
        (
            "nn_to_last_point",
            greedy_algorithms::greedy_nn_to_last_point,
        ),
        ("nn_to_any_point", greedy_algorithms::greedy_nn_to_any_point),
        ("greedy_cycle", greedy_algorithms::greedy_cycle),
        ("nn_to_any_2_regret", regret_heuristics::nn_to_any_2_regret),
        (
            "nn_to_any_weighted_2_regret",
            regret_heuristics::nn_to_any_weighted_2_regret,
        ),
        (
            "greedy_cycle_2_regret",
            regret_heuristics::greedy_cycle_2_regret,
        ),
        (
            "greedy_cycle_weighted_2_regret",
            regret_heuristics::greedy_cycle_weighted_2_regret,
        ),
        // Local Searches
        (
            "ls_greedy_edges_random",
            local_search::ls_greedy_edges_random,
        ),
        (
            "ls_greedy_edges_greedy",
            local_search::ls_greedy_edges_greedy,
        ),
        (
            "ls_greedy_nodes_random",
            local_search::ls_greedy_nodes_random,
        ),
        (
            "ls_greedy_nodes_greedy",
            local_search::ls_greedy_nodes_greedy,
        ),
        (
            "ls_steepest_edges_random",
            local_search::ls_steepest_edges_random,
        ),
        (
            "ls_steepest_edges_greedy",
            local_search::ls_steepest_edges_greedy,
        ),
        (
            "ls_steepest_nodes_random",
            local_search::ls_steepest_nodes_random,
        ),
        (
            "ls_steepest_nodes_greedy",
            local_search::ls_steepest_nodes_greedy,
        ),
        ("ls_candidate_10", local_search_candidates::ls_candidate_10),
        ("ls_candidate_25", local_search_candidates::ls_candidate_25),
        ("ls_candidate_50", local_search_candidates::ls_candidate_50),
        (
            "ls_candidate_10_edge",
            local_search_candidates::ls_candidate_10_edge,
        ),
        (
            "ls_candidate_25_edge",
            local_search_candidates::ls_candidate_25_edge,
        ),
        (
            "ls_candidate_50_edge",
            local_search_candidates::ls_candidate_50_edge,
        ),
        ("msls", multi_local_search::msls),
        ("ils", multi_local_search::ils),
    ])
}

fn map_full()
-> HashMap<&'static str, fn(&Vec<utils::DataPoint>, usize, &Array2<f64>) -> Vec<Vec<usize>>> {
    HashMap::from([
        (
            "ls_greedy_edges_random",
            local_search::ls_greedy_edges_random_full
                as fn(&Vec<utils::DataPoint>, usize, &Array2<f64>) -> Vec<Vec<usize>>,
        ),
        (
            "ls_greedy_edges_greedy",
            local_search::ls_greedy_edges_greedy_full,
        ),
        (
            "ls_greedy_nodes_random",
            local_search::ls_greedy_nodes_random_full,
        ),
        (
            "ls_greedy_nodes_greedy",
            local_search::ls_greedy_nodes_greedy_full,
        ),
        (
            "ls_steepest_edges_random",
            local_search::ls_steepest_edges_random_full,
        ),
        (
            "ls_steepest_edges_greedy",
            local_search::ls_steepest_edges_greedy_full,
        ),
        (
            "ls_steepest_nodes_random",
            local_search::ls_steepest_nodes_random_full,
        ),
        (
            "ls_steepest_nodes_greedy",
            local_search::ls_steepest_nodes_greedy_full,
        ),
        (
            "ls_candidate_10",
            local_search_candidates::ls_candidate_10_full,
        ),
        (
            "ls_candidate_25",
            local_search_candidates::ls_candidate_25_full,
        ),
        (
            "ls_candidate_50",
            local_search_candidates::ls_candidate_50_full,
        ),
    ])
}

#[pyfunction]
fn main(dataset: &str, names: Vec<String>) -> Vec<utils::Metrics> {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let names: Vec<&str> = names.iter().map(|s| &**s).collect();
    let map = get_map();
    let algorithms = Vec::from_iter(names.iter().map(|s| map[s]));
    utils::run_benchmark_suite(algorithms, names, &data, &distance_matrix, false)
}

#[pyfunction]
fn main_mc(dataset: &str, names: Vec<String>) -> Vec<utils::Metrics> {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let names: Vec<&str> = names.iter().map(|s| &**s).collect();
    let map = get_map();
    let algorithms = Vec::from_iter(names.iter().map(|s| map[s]));
    utils::run_benchmark_suite(algorithms, names, &data, &distance_matrix, true)
}

#[pyfunction]
fn complexity(dataset: &str, name: &str) -> Vec<f64> {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let mut times = vec![];
    let map = get_map();
    for i in 2..201 {
        let subset = data[0..i].to_vec();
        let f = map[name];
        let start_time = Instant::now();
        f(&subset, 0, &distance_matrix);
        times.push(start_time.elapsed().as_secs_f64());
    }
    times
}

#[pyfunction]
fn solution_history(dataset: &str, name: &str, point: usize) -> Vec<Vec<usize>> {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let f = map_full()[name];
    f(&data, point, &distance_matrix)
}

#[pyfunction]
fn run(dataset: &str, name: &str) -> utils::Metrics {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    // let names: Vec<&str> = names.iter().map(|s| &**s).collect();
    let map = get_map();
    let algorithms = map[name];
    let start_time = Instant::now();
    let solution = algorithms(&data, 42, &distance_matrix);
    let total_time = start_time.elapsed().as_secs_f64();
    let score = check_solution(&solution, &data, &distance_matrix);
    return Metrics {
        name: name.to_string(),
        scores: vec![score],
        times: vec![total_time],
        best_solution: solution,
    };
}

#[pymodule]
fn evolutionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    m.add_function(wrap_pyfunction!(main_mc, m)?)?;
    m.add_function(wrap_pyfunction!(complexity, m)?)?;
    m.add_function(wrap_pyfunction!(solution_history, m)?)?;
    m.add_function(wrap_pyfunction!(local_search_candidates::assignment_4, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(multi_local_search::assignment_6, m)?)?;
    m.add_class::<Metrics>()?;
    Ok(())
}
