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

fn destroy(
    data: &Vec<DataPoint>,
    solution: Vec<usize>,
    removal_rate: f32,

) -> Vec<usize> {
    let n = solution.len();
    let mut rng = rand::rng();
    let mut new_solution = solution.clone();
    // if rng.random_bool(0.5) {
    //     let mut which_index: Vec<usize> = (0..n).collect();
    //     let hash_solution: HashSet<usize> = HashSet::from_iter(solution.clone());
    //     let all: HashSet<usize> = (0..data.len()).collect();
    //     let mut difference: Vec<usize> = all.difference(&hash_solution).cloned().collect();
    //     which_index.shuffle(&mut rng);
    //     difference.shuffle(&mut rng);
    //     let chosen_indices = &which_index[..inter_mixin_size];
    //     let inter_indices = &difference[..inter_mixin_size];
    //     for (i, j) in chosen_indices.iter().zip(inter_indices) {
    //         new_solution[*i] = *j;
    //     }
    // } else {
    //     let random_start_point = rng.random_range(0..n - intra_mixin_size);
    //     let length = rng.random_range(1..intra_mixin_size);
    //     new_solution[random_start_point..random_start_point + length].shuffle(&mut rng);
    // }
    return new_solution;
}

pub fn large_neighborhood_search_w_ls(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    large_neighborhood_search(data, starting_point_index, distance_matrix, true, 0.3).0
}

pub fn large_neighborhood_search_wo_ls(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    large_neighborhood_search(data, starting_point_index, distance_matrix, false, 0.3).0
}

pub fn large_neighborhood_search(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    apply_local_search: bool,
    removal_rate: f32,
) -> (Vec<usize>, usize) {
    let mut best_solution: Vec<usize> =
        generate_random_solution(data, starting_point_index, distance_matrix);
    let mut best_score: f64 = f64::INFINITY;
    let mut count: usize = 0;
    let max_time = 6.156;
    let start_time = Instant::now();
    while start_time.elapsed().as_secs_f64() < max_time {
        let mut solution = destroy(
            data,
            best_solution.clone(),
            removal_rate,
        );
        if apply_local_search {
            solution = local_search(data, solution, distance_matrix, false, true);
        }
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
pub fn assignment_7(
    dataset: &str,
    removal_rate: f32,
) -> (Vec<usize>, Vec<f64>, Vec<usize>,Vec<usize>, Vec<f64>, Vec<usize>) {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let mut scores: Vec<f64> = vec![];
    let mut counts: Vec<usize> = vec![];
    let mut best_solution: Vec<usize> = vec![];
    let mut best_score = f64::INFINITY;

    let mut scores_ls: Vec<f64> = vec![];
    let mut counts_ls: Vec<usize> = vec![];
    let mut best_solution_ls: Vec<usize> = vec![];
    let mut best_score_ls = f64::INFINITY;
    for _ in 0..20 {
        let (solution, count) = large_neighborhood_search(
            &data,
            42,
            &distance_matrix,
            false,
            removal_rate,
        );
        let score = check_solution(&solution, &data, &distance_matrix);
        counts.push(count);
        scores.push(score);
        if score < best_score {
            best_score = score;
            best_solution = solution;
        }
        let (solution, count) = large_neighborhood_search(
            &data,
            42,
            &distance_matrix,
            true,
            removal_rate,
        );
        let score = check_solution(&solution, &data, &distance_matrix);
        counts_ls.push(count);
        scores_ls.push(score);
        if score < best_score_ls {
            best_score_ls = score;
            best_solution_ls = solution;
        }
    }
    return (
        counts,
        scores,
        best_solution,
        counts_ls,
        scores_ls,
        best_solution_ls,
    );
}
