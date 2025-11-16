use core::f64;
use ndarray::Array2;
use rand::seq::SliceRandom;
use std::collections::HashSet;

use crate::utils::DataPoint;

pub fn intra(solution: &Vec<usize>, i: usize, j: usize, distance_matrix: &Array2<f64>) -> f64 {
    let n = solution.len();
    if i == j {
        return 0.0;
    }

    let (i, j) = if i < j { (i, j) } else { (j, i) };
    if i + 1 == j {
        return -distance_matrix[[solution[(i - 1 + n) % n], solution[i]]]
            - distance_matrix[[solution[j], solution[(j + 1) % n]]]
            + distance_matrix[[solution[(i - 1 + n) % n], solution[j]]]
            + distance_matrix[[solution[i], solution[(j + 1) % n]]];
    } else if (j + 1) % n == i {
        return -distance_matrix[[solution[i], solution[(i + 1) % n]]]
            - distance_matrix[[solution[(j - 1) % n], solution[j]]]
            + distance_matrix[[solution[j], solution[(i + 1) % n]]]
            + distance_matrix[[solution[(j - 1) % n], solution[i]]];
    } else {
        return -distance_matrix[[solution[(i - 1 + n) % n], solution[i]]]
            - distance_matrix[[solution[i], solution[(i + 1) % n]]]
            - distance_matrix[[solution[(j - 1) % n], solution[j]]]
            - distance_matrix[[solution[j], solution[(j + 1) % n]]]
            + distance_matrix[[solution[(i - 1 + n) % n], solution[j]]]
            + distance_matrix[[solution[j], solution[(i + 1) % n]]]
            + distance_matrix[[solution[(j - 1) % n], solution[i]]]
            + distance_matrix[[solution[i], solution[(j + 1) % n]]];
    }
}

pub fn intra_edges(
    solution: &Vec<usize>,
    i: usize,
    j: usize,
    distance_matrix: &Array2<f64>,
) -> f64 {
    let n = solution.len();
    if i == j {
        return 0.0;
    }
    let (i, j) = if i < j { (i, j) } else { (j, i) };
    let (mut a, mut b) = (i, j);
    if a == 0 && b == n - 1 {
        return 0.0;
    }
    if (j + 1) % n == i {
        a = j;
        b = i;
    }
    return -distance_matrix[[solution[(a - 1 + n) % n], solution[a]]]
        + distance_matrix[[solution[(a - 1 + n) % n], solution[b]]]
        - distance_matrix[[solution[b], solution[(b + 1) % n]]]
        + distance_matrix[[solution[a], solution[(b + 1) % n]]];
}

pub fn inter(
    solution: &Vec<usize>,
    i: usize,
    j: usize,
    distance_matrix: &Array2<f64>,
    data: &Vec<DataPoint>,
) -> f64 {
    let n = solution.len();
    return -distance_matrix[[solution[(i - 1 + n) % n], solution[i]]]
        - distance_matrix[[solution[i], solution[(i + 1) % n]]]
        - data[solution[i]].cost as f64
        + distance_matrix[[solution[(i - 1 + n) % n], j]]
        + distance_matrix[[j, solution[(i + 1) % n]]]
        + data[j].cost as f64;
}

fn generate_neighborhood(
    current_solution: &Vec<usize>,
    _data: &Vec<DataPoint>,
) -> Vec<(usize, usize, usize)> {
    let n = current_solution.len();
    let mut solutions: Vec<(usize, usize, usize)> = Vec::new();

    for i in 0..n - 1 {
        for j in i + 1..n {
            solutions.push((0, i, j));
        }
    }

    let hash_solution: HashSet<usize> = HashSet::from_iter(current_solution.clone());
    let m = _data.len();
    let all: HashSet<usize> = (0..m).collect();
    let difference: Vec<usize> = all.difference(&hash_solution).cloned().collect();

    for i in 0..n {
        for j in &difference {
            solutions.push((1, i, *j));
        }
    }
    solutions.shuffle(&mut rand::rng());
    solutions
}

fn search_neighborhood(
    current_solution: &Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    greedy: bool,
    change_edges: bool,
) -> (Vec<usize>, f64) {
    let mut best_solution: Vec<usize> = current_solution.clone();
    let mut best_delta: f64 = 0.0;
    for (index, i, j) in generate_neighborhood(current_solution, data) {
        if index == 0 && change_edges {
            let delta = intra_edges(current_solution, i, j, distance_matrix);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                let sub_slice = &mut new_solution[i..=j];
                sub_slice.reverse();
                (best_delta, best_solution) = (delta, new_solution);
            }
        } else if index == 1 && change_edges {
            let delta = inter(current_solution, i, j, distance_matrix, data);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                new_solution[i] = j;
                (best_delta, best_solution) = (delta, new_solution);
            }
        } else if index == 0 {
            let delta = intra(current_solution, i, j, distance_matrix);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                new_solution.swap(i, j);
                (best_delta, best_solution) = (delta, new_solution);
            }
        } else if index == 1 {
            let delta = inter(current_solution, i, j, distance_matrix, data);
            if delta < best_delta {
                let mut new_solution = current_solution.clone();
                new_solution[i] = j;
                (best_delta, best_solution) = (delta, new_solution);
            }
            if greedy {
                break;
            }
        }
    }
    (best_solution, best_delta)
}

pub fn local_search(
    data: &Vec<DataPoint>,
    initial_solution: Vec<usize>,
    distance_matrix: &Array2<f64>,
    greedy: bool,
    change_edges: bool,
) -> Vec<usize> {
    let mut current_solution = initial_solution.clone();
    loop {
        let (solution, best_delta) = search_neighborhood(
            &current_solution,
            data,
            distance_matrix,
            greedy,
            change_edges,
        );
        if best_delta < 0.0 {
            current_solution = solution;
        } else {
            break;
        }
    }
    current_solution
}

pub fn local_search_full_desc(
    data: &Vec<DataPoint>,
    initial_solution: Vec<usize>,
    distance_matrix: &Array2<f64>,
    greedy: bool,
    change_edges: bool,
) -> Vec<Vec<usize>> {
    let mut full_solution: Vec<Vec<usize>> = Vec::new();
    full_solution.push(initial_solution.clone());
    let mut current_solution = initial_solution.clone();
    loop {
        let (solution, best_delta) = search_neighborhood(
            &current_solution,
            data,
            distance_matrix,
            greedy,
            change_edges,
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
