use ndarray::Array2;

use crate::utils::DataPoint;
use std::{cmp::Ordering, collections::BinaryHeap, cmp::Reverse};

struct Insertion {
    cost: f64,
    position: usize,
}

impl Ord for Insertion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.total_cmp(&other.cost)
    }
}
impl PartialOrd for Insertion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Insertion {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for Insertion { }

fn greedy_cycle_2_regret_pass(
    path:&Vec<usize>,
    candidate:&DataPoint,
    distance_matrix: &Array2<f64>,
) -> (usize,f64,f64){

    let mut priority_queue = BinaryHeap::new();
    for position in 0..path.len() + 1 {
        let (a, b);
        if position == 0 || position == path.len() {
            (a, b) = (path[path.len() - 1], path[0])
        } else {
            (a, b) = (path[position - 1], path[position]);
        }
        let cost = distance_matrix[[a, candidate.id]]
            + distance_matrix[[candidate.id, b]]
            - distance_matrix[[a, b]]
            + candidate.cost as f64;
        priority_queue.push(Reverse(Insertion{cost, position}));
    }
    let first = priority_queue.pop().unwrap();
    let second = priority_queue.pop().unwrap();
    (first.0.position,first.0.cost,second.0.cost-first.0.cost)
}
pub fn greedy_nn_to_any_2_regret_pass(
    path:&Vec<usize>,
    candidate:&DataPoint,
    distance_matrix: &Array2<f64>,
) -> (usize,f64,f64) {
    let mut priority_queue = BinaryHeap::new();
    for position in 0..path.len() + 1 {
        if position == 0 {
            let cost = distance_matrix[[path[0], candidate.id]] + candidate.cost as f64;
            priority_queue.push(Reverse(Insertion{cost, position}));
        } else if position == path.len() {
            let cost = distance_matrix[[path[path.len() - 1], candidate.id]] + candidate.cost as f64;
            priority_queue.push(Reverse(Insertion{cost, position}));
        } else {
            let (a, b) = (path[position - 1], path[position]);
            let cost = distance_matrix[[a, candidate.id]]
            + distance_matrix[[candidate.id, b]]
            - distance_matrix[[a, b]]
            + candidate.cost as f64;
            priority_queue.push(Reverse(Insertion{cost, position}));
        }
    }
    let first = priority_queue.pop().unwrap();
    let second = priority_queue.pop().unwrap();
    (first.0.position,first.0.cost,second.0.cost-first.0.cost)
}

pub fn weighted_regret_heuristic(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    regret_pass: fn(&Vec<usize>,&DataPoint,&Array2<f64>)->(usize,f64,f64),
    weights: [f64;2]
) -> Vec<usize> {
    let starting_point_id = data[starting_point_index].id;
    let mut tsp_path: Vec<usize> = vec![starting_point_id];
    let mut not_visited_points: Vec<DataPoint> = data.clone();
    not_visited_points.remove(starting_point_index);
    for _ in 1..(data.len()+1) / 2 {
        let mut insert_spot: usize = 0;
        let mut best_point_id = starting_point_id;
        let mut min_cost = f64::INFINITY;
        for candidate_point in data {
            let (pos,cost,regret) = regret_pass(&tsp_path, candidate_point, distance_matrix);
            let cost = cost * weights[0] - regret*weights[1];
            if cost < min_cost{
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