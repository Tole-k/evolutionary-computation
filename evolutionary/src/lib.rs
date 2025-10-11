mod utils;
mod greedy_algorithms;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn benchmark(benchmark_name:String) -> PyResult<i32> {
    if benchmark_name == "lab1" {
        Ok(3)
    } else {
        Ok(2)
    }
}

#[pyfunction]
fn main() -> Vec<utils::Metrics>{
    let data: Vec<utils::DataPoint> = utils::load_data("data/TSPA.csv");
    let distance_matrix = utils::calculate_distance_matrix(&data);
    utils::run_benchmark_suite(vec![greedy_algorithms::greedy_nn_to_last_point, greedy_algorithms::greedy_nn_to_any_point, greedy_algorithms::greedy_cycle ],
        vec!["nn_to_last_point", "nn_to_any_point", "greedy_cycle"], &data, &distance_matrix)
}

#[pymodule]
fn evolutionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(benchmark, m)?)?;
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}