use core::f64;
use ndarray::Array2;
use pyo3::pyfunction;
use rand::Rng;
use rand::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

use crate::local_search_base::local_search;
use crate::utils;
use crate::utils::{DataPoint, check_solution, generate_random_solution};

fn mix_solution(
    data: &Vec<DataPoint>,
    solution: Vec<usize>,
    inter_mixin_size: usize,
    intra_mixin_size: usize,
) -> Vec<usize> {
    let n = solution.len();
    let mut rng = rand::rng();
    let mut new_solution = solution.clone();
    if rng.random_bool(0.5) {
        let mut which_index: Vec<usize> = (0..n).collect();
        let hash_solution: HashSet<usize> = HashSet::from_iter(solution.clone());
        let all: HashSet<usize> = (0..data.len()).collect();
        let mut difference: Vec<usize> = all.difference(&hash_solution).cloned().collect();
        which_index.shuffle(&mut rng);
        difference.shuffle(&mut rng);
        let chosen_indices = &which_index[..inter_mixin_size];
        let inter_indices = &difference[..inter_mixin_size];
        for (i, j) in chosen_indices.iter().zip(inter_indices) {
            new_solution[*i] = *j;
        }
    } else {
        let random_start_point = rng.random_range(0..n - intra_mixin_size);
        let length = rng.random_range(1..intra_mixin_size);
        new_solution[random_start_point..random_start_point + length].shuffle(&mut rng);
    }
    return new_solution;
}

pub fn msls(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let mut best_solution: Vec<usize> = vec![];
    let mut best_score: f64 = f64::INFINITY;
    for _ in 0..200 {
        let solution = local_search(
            data,
            generate_random_solution(data, starting_point_index, distance_matrix),
            distance_matrix,
            false,
            true,
        );
        let score = check_solution(&solution, data, distance_matrix);
        if score < best_score {
            best_score = score;
            best_solution = solution;
        }
    }
    best_solution
}

pub fn ils(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let mut best_solution: Vec<usize> =
        generate_random_solution(data, starting_point_index, distance_matrix);
    let mut best_score: f64 = f64::INFINITY;
    let max_time = 6.156;
    let start_time = Instant::now();
    while start_time.elapsed().as_secs_f64() < max_time {
        let solution = local_search(
            data,
            mix_solution(data, best_solution.clone(), 20, 20), // Eventually consider using last solution rather than best solution
            distance_matrix,
            false,
            true,
        );
        let score = check_solution(&solution, data, distance_matrix);
        if score < best_score {
            best_score = score;
            best_solution = solution;
        }
    }
    best_solution
}

pub fn ils_count(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    inter_mixin_size: usize,
    intra_mixin_size: usize,
) -> (Vec<usize>, usize) {
    let mut best_solution: Vec<usize> =
        generate_random_solution(data, starting_point_index, distance_matrix);
    let mut best_score: f64 = f64::INFINITY;
    let mut count: usize = 0;
    let max_time = 6.156;
    let start_time = Instant::now();
    while start_time.elapsed().as_secs_f64() < max_time {
        let solution = local_search(
            data,
            mix_solution(
                data,
                best_solution.clone(),
                inter_mixin_size,
                intra_mixin_size,
            ), // Eventually consider using last solution rather than best solution
            distance_matrix,
            false,
            true,
        );
        count += 1;
        let score = check_solution(&solution, data, distance_matrix);
        if score < best_score {
            best_score = score;
            best_solution = solution;
        }
    }
    (best_solution, count)
}

#[pyfunction]
pub fn assignment_6(
    dataset: &str,
    inter_mixin_size: usize,
    intra_mixin_size: usize,
) -> (Vec<usize>, Vec<f64>, Vec<usize>, Vec<f64>, Vec<usize>) {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let mut scores: Vec<f64> = vec![];
    let mut counts: Vec<usize> = vec![];
    let mut best_solution: Vec<usize> = vec![];
    let mut best_score = f64::INFINITY;

    let mut scores_msls: Vec<f64> = vec![];
    let mut counts_msls: Vec<usize> = vec![];
    let mut best_solution_msls: Vec<usize> = vec![];
    let mut best_score_msls = f64::INFINITY;
    for _ in 0..20 {
        let (solution, count) = ils_count(
            &data,
            42,
            &distance_matrix,
            inter_mixin_size,
            intra_mixin_size,
        );
        let score = check_solution(&solution, &data, &distance_matrix);
        counts.push(count);
        scores.push(score);
        if score < best_score {
            best_score = score;
            best_solution = solution;
        }
        // MSLS
        let solution = msls(&data, 42, &distance_matrix);
        let score = check_solution(&solution, &data, &distance_matrix);
        counts_msls.push(count);
        scores_msls.push(score);
        if score < best_score_msls {
            best_score_msls = score;
            best_solution_msls = solution;
        }
    }
    return (
        counts,
        scores,
        best_solution,
        scores_msls,
        best_solution_msls,
    );
}
