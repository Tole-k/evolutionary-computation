mod greedy_algorithms;
mod regret_heuristics;
mod utils;
use ndarray::Array2;
use pyo3::prelude::*;
use std::{collections::HashMap, time::Instant};

use crate::utils::Metrics;

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
    ])
}

#[pyfunction]
fn main(dataset: &str, names: Vec<String>) -> Vec<utils::Metrics> {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let names: Vec<&str> = names.iter().map(|s| &**s).collect();
    let map = get_map();
    let algorithms = Vec::from_iter(names.iter().map(|s| map[s]));
    utils::run_benchmark_suite(algorithms, names, &data, &distance_matrix)
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

#[pymodule]
fn evolutionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    m.add_function(wrap_pyfunction!(complexity, m)?)?;
    m.add_class::<Metrics>()?;
    Ok(())
}
