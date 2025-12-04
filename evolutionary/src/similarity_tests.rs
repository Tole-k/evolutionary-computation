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
    let set_a: HashSet<[usize; 2]> =
        HashSet::from_iter(solution_a.windows(2).map(|w| [w[0], w[1]]));
    let set_b: HashSet<[usize; 2]> =
        HashSet::from_iter(solution_b.windows(2).map(|w| [w[0], w[1]]));

    return set_a.intersection(&set_b).into_iter().count();
}

#[pyfunction]
pub fn similarity_tests(dataset: &str, measure: &str) -> Vec<(f64, (i32, usize, f64))> {
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
        .max_by(|(_, a), (_, b)| a.1.partial_cmp(&b.1).unwrap())
        .map(|(i, (solution, _))| (solution, i))
        .unwrap();

    solutions
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
                        other_similarities.push(similarity(solution, &other_solution) as f64);
                    }
                });
            let avg_other_similarity =
                other_similarities.iter().sum::<f64>() / other_similarities.iter().count() as f64;
            (*score, (best_similarity, very_good_similarity, avg_other_similarity))
        })
        .collect()
}
