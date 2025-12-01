use crate::local_search_base::{inter, intra, intra_edges};
use crate::utils::{self, DataPoint, Metrics, check_solution, generate_random_solution};
use core::f64;
use ndarray::{Array2, Axis};
use pyo3::pyfunction;
use std::time::Instant;

pub fn build_candidates(
    distance_matrix: &Array2<f64>,
    data: &Vec<DataPoint>,
    size: usize,
) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    let num_cols = distance_matrix.shape()[1];

    for (row_index, row_view) in distance_matrix.axis_iter(Axis(0)).enumerate() {
        let mut scored_neighbors: Vec<(usize, f64)> = (0..num_cols)
            .filter(|&col_index| col_index != row_index)
            .map(|col_index| {
                let distance = row_view[col_index];
                let cost = data[col_index].cost as f64;
                let score = distance + cost;
                (col_index, score)
            })
            .collect();

        scored_neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let best: Vec<usize> = scored_neighbors
            .iter()
            .take(size)
            .map(|&(idx, _)| idx)
            .collect();

        result.push(best);
    }
    result
}

fn generate_neighborhood(
    current_solution: &Vec<usize>,
    _data: &Vec<DataPoint>,
    candidates: &Vec<Vec<usize>>,
) -> Vec<(usize, usize, usize)> {
    let mut solutions: Vec<(usize, usize, usize)> = Vec::new();
    let n = current_solution.len();
    for (i_id, i) in current_solution.iter().enumerate() {
        let prev_node = (i_id - 1 + n) % n;
        let next_node = (i_id + 1) % n;
        for j in candidates[*i].iter() {
            let a = current_solution.iter().position(|&x| x == *j);
            match a {
                Some(index) => {
                    if prev_node != index {
                        solutions.push((0, prev_node, index));
                    }
                    solutions.push((0, i_id, index));
                    if next_node != index {
                        solutions.push((0, next_node, index));
                    }
                }
                None => {
                    solutions.push((1, prev_node, *j));
                    solutions.push((1, i_id, *j));
                    solutions.push((1, next_node, *j));
                }
            }
        }
    }
    solutions
}

fn search_neighborhood(
    current_solution: &Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    change_edges: bool,
    candidates: &Vec<Vec<usize>>,
) -> (Vec<usize>, f64) {
    let mut best_solution: Vec<usize> = current_solution.clone();
    let mut best_delta: f64 = 0.0;
    for (index, i, j) in generate_neighborhood(current_solution, data, candidates) {
        if index == 1 {
            let delta = inter(current_solution, i, j, distance_matrix, data);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                new_solution[i] = j;
                (best_delta, best_solution) = (delta, new_solution);
            }
        } else if change_edges {
            let a: usize;
            let b: usize;
            if i < j {
                a = i;
                b = j;
            } else {
                a = j;
                b = i;
            }
            let delta = intra_edges(current_solution, a, b, distance_matrix);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                new_solution[a..=b].reverse();
                (best_delta, best_solution) = (delta, new_solution);
            }
        } else {
            let delta = intra(current_solution, i, j, distance_matrix);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                new_solution.swap(i, j);
                (best_delta, best_solution) = (delta, new_solution);
            }
        }
    }
    (best_solution, best_delta)
}

pub fn local_search(
    data: &Vec<DataPoint>,
    initial_solution: Vec<usize>,
    distance_matrix: &Array2<f64>,
    change_edges: bool,
    candidates: &Vec<Vec<usize>>,
) -> Vec<usize> {
    let mut current_solution = initial_solution.clone();
    loop {
        let (solution, best_delta) = search_neighborhood(
            &current_solution,
            data,
            distance_matrix,
            change_edges,
            &candidates,
        );
        if best_delta < 0.0 {
            current_solution = solution;
        } else {
            break;
        }
    }
    current_solution
}

pub fn local_search_full(
    data: &Vec<DataPoint>,
    initial_solution: Vec<usize>,
    distance_matrix: &Array2<f64>,
    change_edges: bool,
    candidates: &Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let mut full_solution: Vec<Vec<usize>> = Vec::new();
    full_solution.push(initial_solution.clone());
    let mut current_solution = initial_solution.clone();
    loop {
        let (solution, best_delta) = search_neighborhood(
            &current_solution,
            data,
            distance_matrix,
            change_edges,
            &candidates,
        );
        if best_delta < 0.0 {
            full_solution.push(solution.clone());
            current_solution = solution;
        } else {
            break;
        }
    }
    full_solution
}

pub fn ls_candidate_10(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 10);
    local_search(data, initial_solution, distance_matrix, false, &candidates)
}
pub fn ls_candidate_25(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 25);
    local_search(data, initial_solution, distance_matrix, false, &candidates)
}
pub fn ls_candidate_50(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 50);
    local_search(data, initial_solution, distance_matrix, false, &candidates)
}
pub fn ls_candidate_10_edge(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 10);
    local_search(data, initial_solution, distance_matrix, true, &candidates)
}
pub fn ls_candidate_25_edge(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 25);
    local_search(data, initial_solution, distance_matrix, true, &candidates)
}
pub fn ls_candidate_50_edge(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 50);
    local_search(data, initial_solution, distance_matrix, true, &candidates)
}

pub fn ls_candidate_10_full(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<Vec<usize>> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 10);
    local_search_full(data, initial_solution, distance_matrix, false, &candidates)
}
pub fn ls_candidate_25_full(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<Vec<usize>> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 25);
    local_search_full(data, initial_solution, distance_matrix, false, &candidates)
}
pub fn ls_candidate_50_full(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<Vec<usize>> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    let candidates = build_candidates(distance_matrix, data, 50);
    local_search_full(data, initial_solution, distance_matrix, false, &candidates)
}

pub fn ls_candidate_faster(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    candidates: &Vec<Vec<usize>>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, true, &candidates)
}

#[pyfunction]
pub fn assignment_4(dataset: &str, size: usize) -> Metrics {
    let data: Vec<DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let mut scores: Vec<f64> = vec![];
    let mut best_solution_score: f64 = f64::INFINITY;
    let mut best_solution: Vec<usize> = vec![];

    let mut times = vec![];
    let candidates = build_candidates(&distance_matrix, &data, size);
    for i in 0..data.len() {
        let start_time = Instant::now();
        let solution = ls_candidate_faster(&data, i, &distance_matrix, &candidates);
        times.push(start_time.elapsed().as_secs_f64());
        let solution_score = check_solution(&solution, &data, &distance_matrix);
        scores.push(solution_score);
        if solution_score < best_solution_score {
            best_solution_score = solution_score;
            best_solution = solution;
        }
    }
    let name = "local_search_candidates".to_string();
    Metrics {
        name,
        scores,
        times,
        best_solution,
    }
}
