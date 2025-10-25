use core::f64;
use ndarray::Array2;
use rand::seq::SliceRandom;
use std::collections::HashSet;

use crate::utils::{DataPoint, check_solution};

pub fn intra_neighbors(solution: &Vec<usize>) -> Vec<Vec<usize>> {
    // n^2 complexity: 50 000 solutions per call
    let n = solution.len();
    let mut solutions: Vec<Vec<usize>> = vec![];
    for i in 0..n - 1 {
        for j in i + 1..n {
            let mut new_solution = solution.clone();
            new_solution.swap(i, j);
            solutions.push(new_solution);
        }
    }
    solutions
}

pub fn inter_neighbors(solution: &Vec<usize>, data: &Vec<DataPoint>) -> Vec<Vec<usize>> {
    let n = solution.len();
    let hash_solution: HashSet<usize> = HashSet::from_iter(solution.clone());
    let m = data.len();
    let all: HashSet<usize> = (0..m).collect();
    let difference: Vec<usize> = all.difference(&hash_solution).cloned().collect();

    let mut solutions: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        for j in &difference {
            let mut new_solution = solution.clone();
            new_solution[i] = *j;
            solutions.push(new_solution);
        }
    }
    solutions
}

fn generate_neighborhood(current_solution: &Vec<usize>, _data: &Vec<DataPoint>) -> Vec<Vec<usize>> {
    let mut intra = intra_neighbors(current_solution);
    let mut inter = inter_neighbors(current_solution, _data);
    intra.append(&mut inter);
    intra.shuffle(&mut rand::rng());
    intra
}

fn search_neighborhood(
    current_solution: &Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    greedy: bool,
) -> (Vec<usize>, f64) {
    let mut best_solution: Vec<usize> = vec![];
    let mut best_score: f64 = f64::INFINITY;
    for solution in generate_neighborhood(current_solution, data) {
        let score: f64 = check_solution(&solution, data, distance_matrix);
        if score < best_score {
            (best_score, best_solution) = (score, solution);
            if greedy {
                break;
            }
        }
    }
    (best_solution, best_score)
}

pub fn local_search(
    data: &Vec<DataPoint>,
    initial_solution: Vec<usize>,
    distance_matrix: &Array2<f64>,
    greedy: bool,
) -> Vec<usize> {
    let mut current_solution = initial_solution.clone();
    let mut current_score = check_solution(&current_solution, data, distance_matrix);
    let mut better_solution_found: bool = true;
    while better_solution_found {
        let (solution, best_score) =
            search_neighborhood(&current_solution, data, distance_matrix, greedy);

        if best_score < current_score {
            (current_solution, current_score) = (solution.clone(), best_score);
        } else {
            better_solution_found = false;
        }
    }
    current_solution
}
