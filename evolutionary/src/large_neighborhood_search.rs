use core::f64;
use ndarray::Array2;
use pyo3::pyfunction;
use rand::Rng;
use std::time::Instant;

use crate::local_search_base::local_search;
use crate::utils;
use crate::utils::{DataPoint, check_solution, generate_random_solution};
use crate::regret_heuristics::nn_to_any_2_regret_pass;

struct RouletteWheel {
    items:Vec<(usize, f32)>,
}

impl RouletteWheel {
    pub fn total_weight(&self)->f32{
        self.items.iter().map(|a|a.1).sum()
    }
    pub fn sample(&mut self)->(usize,f32){
        let mut rng = rand::rng();
        let random = rng.random_range(0.0..self.total_weight());
        let mut accumulate= 0.0;
        let mut i =0;
        loop {
            accumulate+=self.items[i].1 ;
            if accumulate>random{
                break
            }else{
                i+=1;
            }
        }
        self.items.remove(i)
    }
}

fn destroy(
    data: &Vec<DataPoint>,
    solution: Vec<usize>,
    removal_rate: f32,
) -> Vec<usize> {
    let n = solution.len();
    let mut new_solution = solution.clone();
    let to_remove = (removal_rate * n as f32).ceil() as usize;
    let mut node_vec: Vec<(usize,i32)>=vec![];
    for (i, &node_id) in solution.iter().enumerate(){
        let node = data[node_id];
        node_vec.push((i,node.cost));
    }
    let mut roulette_wheel = RouletteWheel{items:node_vec.iter().map(|a|(a.0,a.1 as f32)).collect()};
    for _ in 0..to_remove{
        let (i, _) = roulette_wheel.sample();
        new_solution.remove(i);
    }
    // new_solution = roulette_wheel.items.iter().map(|a|a.0).collect();
    return new_solution;
}

fn regret_from_partial_solution(
    data: &Vec<DataPoint>,
    initial_solution: &Vec<usize>,
    distance_matrix: &Array2<f64>,
    regret_pass: fn(&Vec<usize>, &DataPoint, &Array2<f64>) -> (usize, f64, f64),
    weights: [f64; 2],
) -> Vec<usize> {
    let mut tsp_path: Vec<usize> = initial_solution.clone();
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    for &id in initial_solution {
        let index = not_visited_points
            .iter()
            .position(|n| n.id == id)
            .unwrap();
        not_visited_points.remove(index);
    }
    for _ in 1..(data.len() + 1) / 2 {
        let mut insert_spot: usize = 0;
        let mut best_point_id = initial_solution[0];
        let mut min_cost = f64::INFINITY;
        for candidate_point in &not_visited_points {
            let (pos, cost, regret) = regret_pass(&tsp_path, candidate_point, distance_matrix);
            let cost = cost * weights[0] - regret * weights[1];
            if cost < min_cost {
                min_cost = cost;
                insert_spot = pos;
                best_point_id = candidate_point.id;
            }
        }
        tsp_path.insert(insert_spot, best_point_id);
        let index = not_visited_points
            .iter()
            .position(|n| n.id == best_point_id)
            .unwrap();
        not_visited_points.remove(index);
    }
    tsp_path
}

fn repair(data: &Vec<DataPoint>,
    solution: Vec<usize>,distance_matrix: &Array2<f64>)-> Vec<usize> {
        regret_from_partial_solution(data, &solution, distance_matrix, nn_to_any_2_regret_pass, [0.5,0.5])
    }

pub fn large_neighborhood_search_w_ls(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    large_neighborhood_search(data, starting_point_index, distance_matrix, true, 0.3, 6.156).0
}

pub fn large_neighborhood_search_wo_ls(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    large_neighborhood_search(data, starting_point_index, distance_matrix, false, 0.3, 6.156).0
}

pub fn large_neighborhood_search(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    apply_local_search: bool,
    removal_rate: f32,
    max_time: f64,
) -> (Vec<usize>, usize) {
    let mut best_solution: Vec<usize> = generate_random_solution(data, starting_point_index, distance_matrix);
    best_solution = local_search(data, best_solution, distance_matrix, false, true);
    let mut best_score: f64 = check_solution(&best_solution, data, distance_matrix);
    let mut count: usize = 0;
    let start_time = Instant::now();
    while start_time.elapsed().as_secs_f64() < max_time {
        let mut solution = destroy(
            data,
            best_solution.clone(),
            removal_rate,
        );
        solution = repair(data, solution, distance_matrix);
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
    let max_time = 1.156;
    let mut scores: Vec<f64> = vec![];
    let mut counts: Vec<usize> = vec![];
    let mut best_solution: Vec<usize> = vec![];
    let mut best_score = f64::INFINITY;

    let mut scores_ls: Vec<f64> = vec![];
    let mut counts_ls: Vec<usize> = vec![];
    let mut best_solution_ls: Vec<usize> = vec![];
    let mut best_score_ls = f64::INFINITY;
    for _ in 0..5 {
        let (solution, count) = large_neighborhood_search(
            &data,
            42,
            &distance_matrix,
            false,
            removal_rate,
            max_time,
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
            false,
            removal_rate,
            max_time,
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
