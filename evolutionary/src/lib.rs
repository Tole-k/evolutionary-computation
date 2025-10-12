mod greedy_algorithms;
mod utils;
use ndarray::Array2;
use pyo3::prelude::*;
use std::{collections::HashMap, time::Instant};

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
    ])
}

#[pyfunction]
fn benchmark(benchmark_name: String) -> PyResult<i32> {
    if benchmark_name == "lab1" {
        Ok(3)
    } else {
        Ok(2)
    }
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
    for i in [10, 20, 50, 100, 100, 200] {
        let subset = data[0..i].to_vec();
        let mut total_time = 0.0;
        for i in 0..subset.len() {
            let f = map[name];
            let start_time = Instant::now();
            f(&subset, i, &distance_matrix);
            total_time += start_time.elapsed().as_secs_f64();
        }
        times.push(total_time);
    }
    times
}

#[pymodule]
fn evolutionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(benchmark, m)?)?;
    m.add_function(wrap_pyfunction!(main, m)?)?;
    m.add_function(wrap_pyfunction!(complexity, m)?)?;
    Ok(())
}
