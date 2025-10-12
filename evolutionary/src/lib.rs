mod greedy_algorithms;
mod utils;
use pyo3::prelude::*;
use std::collections::HashMap;
use ndarray::Array2;

#[pyfunction]
fn benchmark(benchmark_name: String) -> PyResult<i32> {
    if benchmark_name == "lab1" {
        Ok(3)
    } else {
        Ok(2)
    }
}
#[pyfunction]
fn main(dataset: &str,names: Vec<String>) -> Vec<utils::Metrics> {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let names: Vec<&str> = names.iter().map(|s| &**s).collect();
    let mut map: HashMap<&str, fn(&Vec<utils::DataPoint>, usize, &Array2<f64>) -> Vec<usize>> = HashMap::new();
    map.insert("random", utils::generate_random_solution);
    map.insert("nn_to_last_point",greedy_algorithms::greedy_nn_to_last_point);
    map.insert("nn_to_any_point",greedy_algorithms::greedy_nn_to_any_point);
    map.insert("greedy_cycle", greedy_algorithms::greedy_cycle);
    let algorithms =Vec::from_iter(names.iter().map(|s| map[s]));
    utils::run_benchmark_suite(
        algorithms,
        names,
        &data,
        &distance_matrix,
    )
}

#[pymodule]
fn evolutionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(benchmark, m)?)?;
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}