use crate::local_search_base::{inter, intra, intra_edges};
use crate::utils::{DataPoint, generate_random_solution};
use core::f64;
use ndarray::{Array2, Axis};
use rand::seq::SliceRandom;

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

    for i in current_solution.iter() {
        for j in candidates[*i].iter() {
            solutions.push((!current_solution.contains(j) as usize, *i, *j));
        }
    }

    solutions.shuffle(&mut rand::rng());
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
        let a = current_solution
            .iter()
            .position(|&x| x == i)
            .expect("This shouldn't happen");
        if index == 1 {
            let delta = inter(current_solution, a, j, distance_matrix, data);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                new_solution[a] = j;
                (best_delta, best_solution) = (delta, new_solution);
            }
        } else if change_edges {
            let delta = intra_edges(current_solution, i, j, distance_matrix);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                let sub_slice = &mut new_solution[i..=j];
                sub_slice.reverse();
                (best_delta, best_solution) = (delta, new_solution);
            }
        } else {
            let b = current_solution
                .iter()
                .position(|&x| x == j)
                .expect("This shouldn't happen");
            let delta = intra(current_solution, a, b, distance_matrix);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                new_solution.swap(a, b);
                (best_delta, best_solution) = (delta, new_solution);
                // For some reason it isn't swapping
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

pub fn ls_candidate_faster(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    candidates: &Vec<Vec<usize>>,
) -> Vec<usize> {
    let initial_solution = generate_random_solution(data, starting_point_index, distance_matrix);
    local_search(data, initial_solution, distance_matrix, false, &candidates)
}
