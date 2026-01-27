use std::collections::HashSet;

use pyo3::pyfunction;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    local_search_base::local_search,
    multi_local_search::ils,
    utils::{self, check_solution, generate_random_solution},
};

fn node_similarity(solution_a: &Vec<usize>, solution_b: &Vec<usize>) -> usize {
    let set_a: HashSet<&usize> = HashSet::from_iter(solution_a);
    let set_b: HashSet<&usize> = HashSet::from_iter(solution_b);

    return set_a.intersection(&set_b).into_iter().count();
}

fn edge_similarity(solution_a: &Vec<usize>, solution_b: &Vec<usize>) -> usize {
    let mut set_a: HashSet<[usize; 2]> = HashSet::new();
    for i in 0..solution_a.len() {
        let a = solution_a[i];
        let b = solution_a[(i + 1) % solution_a.len()];
        set_a.insert(if a < b { [a, b] } else { [b, a] });
    }

    let mut set_b: HashSet<[usize; 2]> = HashSet::new();
    for i in 0..solution_b.len() {
        let a = solution_b[i];
        let b = solution_b[(i + 1) % solution_b.len()];
        set_b.insert(if a < b { [a, b] } else { [b, a] });
    }

    return set_a.intersection(&set_b).into_iter().count();
}

fn correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    let sum_x2: f64 = x.iter().map(|a| a * a).sum();
    let sum_y2: f64 = y.iter().map(|a| a * a).sum();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

#[pyfunction]
pub fn similarity_tests(
    dataset: &str,
    measure: &str,
) -> (Vec<(f64, (i32, usize, f64))>, (f64, f64, f64)) {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("data/{dataset}.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let num_solutions = 1_000;
    let similarity = match measure {
        "node" => node_similarity,
        "edge" => edge_similarity,
        _ => panic!("Invalid measure"),
    };

    let solutions: Vec<(Vec<usize>, f64)> = (0..num_solutions)
        .collect::<Vec<i32>>()
        .par_iter()
        .with_min_len(1)
        .map(|&_i| {
            let random = generate_random_solution(&data, 42, &distance_matrix);
            let solution = local_search(&data, random, &distance_matrix, true, true);
            let score = check_solution(&solution, &data, &distance_matrix);
            (solution, score)
        })
        .collect();

    let very_good_solution = ils(&data, 42, &distance_matrix);
    let (best_solution, best_solution_position) = solutions
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.1.partial_cmp(&b.1).unwrap())
        .map(|(i, (solution, _))| (solution, i))
        .unwrap();

    let results: Vec<(f64, (i32, usize, f64))> = solutions
        .clone()
        .par_iter()
        .with_min_len(1)
        .enumerate()
        .map(|(i, (solution, score))| {
            let solution = solution;
            let best = best_solution;
            let very_good = &very_good_solution;
            let best_similarity = if best_solution_position == i {
                -1
            } else {
                similarity(best, solution) as i32
            };
            let very_good_similarity = similarity(solution, very_good);
            let mut other_similarities = vec![];
            solutions
                .iter()
                .enumerate()
                .for_each(|(j, (other_solution, _))| {
                    if i != j {
                        other_similarities.push(similarity(solution, other_solution) as f64);
                    }
                });
            let avg_other_similarity =
                other_similarities.iter().sum::<f64>() / other_similarities.iter().count() as f64;
            (
                *score,
                (best_similarity, very_good_similarity, avg_other_similarity),
            )
        })
        .collect();

    let scores: Vec<f64> = results.iter().map(|(s, _)| *s).collect();

    let (scores_best, similarities_best): (Vec<f64>, Vec<f64>) = results
        .iter()
        .filter(|(_, (best_sim, _, _))| *best_sim != -1)
        .map(|(s, (best_sim, _, _))| (*s, *best_sim as f64))
        .unzip();
    let corr_best = correlation(&scores_best, &similarities_best);

    let similarities_very_good: Vec<f64> =
        results.iter().map(|(_, (_, vg, _))| *vg as f64).collect();
    let corr_very_good = correlation(&scores, &similarities_very_good);

    let similarities_avg: Vec<f64> = results.iter().map(|(_, (_, _, avg))| *avg).collect();
    let corr_avg = correlation(&scores, &similarities_avg);

    (results, (corr_best, corr_very_good, corr_avg))
}
