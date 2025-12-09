use crate::local_search;
use crate::local_search_base::local_search;
use crate::utils::{self, DataPoint, check_solution};
use core::f64;
use ndarray::Array2;
use ordered_float::OrderedFloat;
use rand::seq::IteratorRandom;
use rand::{Rng, rng};
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Solution {
    pub score: OrderedFloat<f64>,
    pub path: Vec<usize>,
}

impl Solution {
    pub fn new(path: Vec<usize>, score: f64) -> Self {
        Self {
            score: OrderedFloat(score),
            path,
        }
    }
    pub fn worst() -> Self {
        Self {
            score: OrderedFloat(f64::INFINITY),
            path: Vec::new(),
        }
    }
}

struct ElitePopulation {
    heap: BinaryHeap<Solution>,
    data: Vec<DataPoint>,
    distance_matrix: Array2<f64>,
    capacity: usize,
}

impl ElitePopulation {
    pub fn new(data: Vec<DataPoint>, distance_matrix: Array2<f64>) -> Self {
        Self {
            heap: BinaryHeap::new(),
            data: data,
            distance_matrix: distance_matrix,
            capacity: 20,
        }
    }
    fn contains_path(&self, path: &[usize]) -> bool {
        self.heap.iter().any(|s| s.path == path)
    }

    pub fn push(&mut self, path: Vec<usize>) {
        let score = check_solution(&path, &self.data, &self.distance_matrix);
        let candidate = Solution::new(path, score);

        if self.heap.len() < self.capacity {
            self.heap.push(candidate);
            return;
        }

        if self.contains_path(&candidate.path) {
            return;
        }

        if let Some(worst) = self.heap.peek() {
            if candidate.score >= worst.score {
                return;
            }
        }

        self.heap.pop();
        self.heap.push(candidate);
    }

    pub fn pop(&mut self) -> Solution {
        self.heap.pop().unwrap_or_else(Solution::worst)
    }

    pub fn get_parents(&self) -> (Vec<usize>, Vec<usize>) {
        let mut rng = rng();

        let mut sampled = self.heap.iter().choose_multiple(&mut rng, 2);

        let a = sampled.pop().map(|s| s.path.clone()).unwrap_or_default();
        let b = sampled.pop().map(|s| s.path.clone()).unwrap_or_default();

        (a, b)
    }
}

fn operator_1(parent1: Vec<usize>, parent2: Vec<usize>, data: &Vec<DataPoint>) -> Vec<usize> {
    let all: HashSet<usize> = (0..data.len()).collect();
    let hash_parent_2: HashSet<usize> = HashSet::from_iter(parent2.clone());
    let mut subpaths: Vec<Vec<usize>> = Vec::new();
    let mut new_path = true;
    for &i in &parent1 {
        if hash_parent_2.contains(&i) {
            new_path = true;
            continue;
        }
        if new_path {
            subpaths.push(vec![i]);
            new_path = false
        } else {
            subpaths
                .last_mut()
                .expect("Subpath vector do not exist, and we are adding element to it")
                .push(i);
        }
    }
    // TODO: Merge
    return vec![];
}

fn operator_2(
    parent1: Vec<usize>,
    parent2: Vec<usize>,
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    // TODO: Implement Operator
    let hash_parent_1: HashSet<usize> = HashSet::from_iter(parent1.clone());
    let hash_parent_2: HashSet<usize> = HashSet::from_iter(parent2.clone());
    let mut union: Vec<usize> = hash_parent_1.union(&hash_parent_2).cloned().collect();
    // TODO: Repair with heuristic
    return vec![];
}

fn initiate_population(data: &Vec<DataPoint>, distance_matrix: &Array2<f64>) -> ElitePopulation {
    let mut population: ElitePopulation =
        ElitePopulation::new(data.clone(), distance_matrix.clone());
    for _ in 0..20 {
        let solution = local_search::ls_greedy_edges_random(data, 0, distance_matrix);
        population.push(solution);
    }
    population
}

pub fn hybrid_algorithm(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
) -> Vec<usize> {
    let _ = starting_point_index;
    let max_time = 6.156;
    let start_time = Instant::now();
    let mut population = initiate_population(data, distance_matrix);
    let mut rng = rand::rng();
    while start_time.elapsed().as_secs_f64() < max_time {
        let (parent1, parent2) = population.get_parents();
        let mut child;
        if rng.random_bool(0.5) {
            child = operator_1(parent1, parent2, data);
        } else {
            child = operator_2(parent1, parent2, data, distance_matrix);
        }
        child = local_search(data, child, distance_matrix, false, true);
        population.push(child);
    }
    population.pop().path
}

pub fn test() {
    let data: Vec<utils::DataPoint> = utils::load_data(&format!("../data/TSPA.csv"));
    let distance_matrix = utils::calculate_distance_matrix(&data);
    let mut population = initiate_population(&data, &distance_matrix);
}
