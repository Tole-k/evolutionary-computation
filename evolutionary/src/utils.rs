use crate::local_search_candidates::build_candidates;
use core::f64;
use csv::ReaderBuilder;
use ndarray::{Array1, Array2, Axis};
use pyo3::{pyclass, pymethods};
use rand::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

#[derive(Copy, Clone)]
pub struct DataPoint {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub cost: i32,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct Metrics {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub scores: Vec<f64>,
    #[pyo3(get, set)]
    pub times: Vec<f64>,
    #[pyo3(get, set)]
    pub best_solution: Vec<usize>,
}

#[pymethods]
impl Metrics {
    #[new]
    fn new(name: String, scores: Vec<f64>, times: Vec<f64>, best_solution: Vec<usize>) -> Self {
        Self {
            name,
            scores,
            times,
            best_solution,
        }
    }
}

pub fn calculate_distance_matrix(records: &Vec<DataPoint>) -> Array2<f64> {
    let records = Array1::from_vec(records.clone());
    let x = records.map(|s| s.x as f64);
    let y = records.map(|s| s.y as f64);
    let a_x = &x.clone().insert_axis(Axis(1));
    let b_x = &x.insert_axis(Axis(0));
    let a_y = &y.clone().insert_axis(Axis(1));
    let b_y = &y.insert_axis(Axis(0));
    (((a_x - b_x).pow2() + (a_y - b_y).pow2()).sqrt()).round()
}

pub fn load_data(path: &str) -> Vec<DataPoint> {
    let reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(path);
    let mut records_mut: Vec<DataPoint> = vec![];

    for (id, record) in reader.unwrap().records().enumerate() {
        let uwrapped_record = record.unwrap();
        let x: i32 = FromStr::from_str(uwrapped_record.get(0).unwrap()).unwrap();
        let y: i32 = FromStr::from_str(uwrapped_record.get(1).unwrap()).unwrap();
        let cost: i32 = FromStr::from_str(uwrapped_record.get(2).unwrap()).unwrap();
        records_mut.push(DataPoint { id, x, y, cost });
    }
    records_mut
}

pub fn check_solution(
    solution: &Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
) -> f64 {
    let mut total_value = 0.0;
    let first_point = data[solution[0]];
    let mut last_point = first_point;
    for index in 1..solution.len() {
        let current_point = data[solution[index]];
        total_value += distance_matrix[[last_point.id, current_point.id]];
        total_value += current_point.cost as f64;
        last_point = current_point;
    }
    total_value += distance_matrix[[last_point.id, first_point.id]] + first_point.cost as f64;
    total_value
}

pub fn generate_random_solution(
    data: &Vec<DataPoint>,
    _starting_point_index: usize,
    _distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let size = data.len();
    let mut nums: Vec<usize> = (0..size).collect();
    let mut rng = rand::rng();
    nums.shuffle(&mut rng);
    let half_nums = &nums[..size / 2];
    half_nums.to_vec()
}

pub fn benchmark_function(
    f: fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    name: &str,
) -> Metrics {
    let mut scores: Vec<f64> = vec![];
    let mut best_solution_score: f64 = f64::INFINITY;
    let mut best_solution: Vec<usize> = vec![];

    let mut times = vec![];
    for i in 0..data.len() {
        let start_time = Instant::now();
        let solution = f(data, i, distance_matrix);
        times.push(start_time.elapsed().as_secs_f64());
        let solution_score = check_solution(&solution, data, distance_matrix);
        scores.push(solution_score);
        if solution_score < best_solution_score {
            best_solution_score = solution_score;
            best_solution = solution;
        }
    }
    let name = name.to_string();
    Metrics {
        name,
        scores,
        times,
        best_solution,
    }
}

pub fn benchmark_function_alpha(
    f: fn(&Vec<DataPoint>, usize, &Array2<f64>, &Vec<Vec<usize>>) -> Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    name: &str,
    size: usize,
) -> Metrics {
    let mut scores: Vec<f64> = vec![];
    let mut best_solution_score: f64 = f64::INFINITY;
    let mut best_solution: Vec<usize> = vec![];

    let mut times = vec![];
    let candidates = build_candidates(distance_matrix, data, size);
    for i in 0..data.len() {
        let start_time = Instant::now();
        let solution = f(data, i, distance_matrix, &candidates);
        times.push(start_time.elapsed().as_secs_f64());
        let solution_score = check_solution(&solution, data, distance_matrix);
        scores.push(solution_score);
        if solution_score < best_solution_score {
            best_solution_score = solution_score;
            best_solution = solution;
        }
    }
    let name = name.to_string();
    Metrics {
        name,
        scores,
        times,
        best_solution,
    }
}

pub fn benchmark_function_mc(
    f: fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    name: &str,
) -> Metrics {
    let mut scores: Vec<f64> = vec![];
    let mut times: Vec<f64> = vec![];
    let mut best_solution_score: f64 = f64::INFINITY;
    let mut best_solution: Vec<usize> = vec![];

    let results: Vec<(f64, f64, Vec<usize>)> = (0..data.len())
        .collect::<Vec<usize>>()
        .par_iter()
        .with_min_len(25)
        .map(|&i| {
            let start_time = Instant::now();
            let solution = f(data, i, distance_matrix);
            (
                check_solution(&solution, data, distance_matrix),
                start_time.elapsed().as_secs_f64(),
                solution,
            )
        })
        .collect();

    for (score, time, solution) in results {
        scores.push(score);
        times.push(time);
        if score < best_solution_score {
            best_solution_score = score;
            best_solution = solution;
        }
    }
    let name = name.to_string();
    Metrics {
        name,
        scores,
        times,
        best_solution,
    }
}

pub fn run_benchmark_suite(
    functions: Vec<fn(&Vec<DataPoint>, usize, &Array2<f64>) -> Vec<usize>>,
    names: Vec<&str>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    mc: bool,
) -> Vec<Metrics> {
    let mut results: Vec<Metrics> = vec![];
    for iter_tuple in functions.iter().zip(names.iter()) {
        let (function, name) = iter_tuple;
        if mc {
            results.push(benchmark_function_mc(
                *function,
                data,
                distance_matrix,
                name,
            ));
        } else {
            results.push(benchmark_function(*function, data, distance_matrix, name));
        }
    }
    let mut old_json: HashMap<String, Metrics> = match fs::read_to_string("result.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => HashMap::new(),
    };
    let new_json: HashMap<String, &Metrics> = results.iter().map(|m| (m.name.clone(), m)).collect();
    for (k, v) in new_json {
        old_json.insert(k, v.clone());
    }
    let map_as_json = serde_json::to_string_pretty(&old_json).unwrap();
    let mut file = File::create("result.json").expect("Could not create file!");

    file.write_all(map_as_json.as_bytes())
        .expect("Cannot write to the file!");
    if results.is_empty() {
        results = vec![Metrics {
            name: "test".to_string(),
            scores: vec![1.1],
            times: vec![1.1],
            best_solution: vec![1],
        }]
    }
    results
}
