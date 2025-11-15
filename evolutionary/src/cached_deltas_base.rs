use crate::local_search_base::{generate_neighborhood, inter, intra, intra_edges};
use crate::utils::DataPoint;
use crate::utils::generate_random_solution;
use ndarray::Array2;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

trait Move {
    fn delta(&self) -> f64;
}
#[derive(Clone)]
struct IntraEdgeMove {
    delta: f64,
    edge1: [usize; 2],
    edge2: [usize; 2],
}
#[derive(Clone)]
struct IntraNodeMove {
    delta: f64,
    node1_w_neighbors: [usize; 3],
    node2_w_neighbors: [usize; 3],
}

#[derive(Clone)]
struct InterNodeMove {
    delta: f64,
    node1_w_neighbors: [usize; 3],
    outside_node: usize,
}
#[derive(Clone)]
enum MoveType {
    IntraEdge(IntraEdgeMove),
    IntraNode(IntraNodeMove),
    InterNode(InterNodeMove),
}
impl Eq for MoveType {}

impl PartialEq for MoveType {
    fn eq(&self, other: &Self) -> bool {
        self.delta() == other.delta()
    }
}

impl PartialOrd for MoveType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.delta().partial_cmp(&other.delta())
    }
}
impl Ord for MoveType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.delta().total_cmp(&other.delta())
    }
}

impl Move for MoveType {
    fn delta(&self) -> f64 {
        match self {
            MoveType::IntraEdge(m) => m.delta,
            MoveType::IntraNode(m) => m.delta,
            MoveType::InterNode(m) => m.delta,
        }
    }
}

fn apply_if_valid(
    current_solution: &Vec<usize>,
    stored_moves: &mut Vec<MoveType>,
    mv: MoveType,
) -> Option<(f64, Vec<usize>)> {
    match mv {
        MoveType::IntraEdge(IntraEdgeMove {
            delta,
            edge1,
            edge2,
        }) => match case_intra_edges(current_solution, mv, delta, edge1, edge2, stored_moves) {
            None => return None,
            Some(value) => return Some(value),
        },
        MoveType::IntraNode(IntraNodeMove {
            delta,
            node1_w_neighbors,
            node2_w_neighbors,
        }) => {
            match case_intra_nodes(
                current_solution,
                delta,
                node1_w_neighbors,
                node2_w_neighbors,
            ) {
                None => return None,
                Some(value) => return Some(value),
            }
        }
        MoveType::InterNode(InterNodeMove {
            delta,
            node1_w_neighbors,
            outside_node,
        }) => match case_inter_nodes(
            current_solution,
            delta,
            node1_w_neighbors,
            outside_node,
        ) {
            None => return None,
            Some(value) => return Some(value),
        },
    }
}

fn case_inter_nodes(
    current_solution: &Vec<usize>,
    delta: f64,
    node1_w_neighbors: [usize; 3],
    outside_node: usize,
) -> Option<(f64, Vec<usize>)> {
    let [prev_node1, node1, node1_next] = node1_w_neighbors;
    let node1_position: usize;
    let prev_node1_position: usize;
    let node1_next_position: usize;
    match current_solution.iter().position(|x| *x == node1) {
        None => {
            return None;
        }
        Some(position) => {
            node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == prev_node1) {
        None => {
            return None;
        }
        Some(position) => {
            prev_node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == node1_next) {
        None => {
            return None;
        }
        Some(position) => {
            node1_next_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == outside_node) {
        None => {}
        Some(_position) => {
            return None;
        }
    }
    if prev_node1_position + 1 == node1_position && node1_position == node1_next_position - 1 {
        let mut new_solution = current_solution.clone();
        new_solution[node1_position] = outside_node;
        return Some((delta, new_solution));
    } else {
        return None;
    }
}

fn case_intra_nodes(
    current_solution: &Vec<usize>,
    delta: f64,
    node1_w_neighbors: [usize; 3],
    node2_w_neighbors: [usize; 3],
) -> Option<(f64, Vec<usize>)> {
    let [prev_node1, node1, node1_next] = node1_w_neighbors;
    let [prev_node2, node2, node2_next] = node2_w_neighbors;
    let node1_position: usize;
    let prev_node1_position: usize;
    let node1_next_position: usize;
    let prev_node2_position: usize;
    let node2_position: usize;
    let node2_next_position: usize;
    match current_solution.iter().position(|x| *x == node1) {
        None => {
            return None;
        }
        Some(position) => {
            node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == prev_node1) {
        None => {
            return None;
        }
        Some(position) => {
            prev_node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == node1_next) {
        None => {
            return None;
        }
        Some(position) => {
            node1_next_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == node2) {
        None => {
            return None;
        }
        Some(position) => {
            node2_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == prev_node2) {
        None => {
            return None;
        }
        Some(position) => {
            prev_node2_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == node2_next) {
        None => {
            return None;
        }
        Some(position) => {
            node2_next_position = position;
        }
    }
    if (prev_node1_position + 1 == node1_position && node1_position == node1_next_position - 1)
        && (prev_node2_position + 1 == node2_position && node2_position == node2_next_position - 1)
    {
        let mut new_solution = current_solution.clone();
        new_solution.swap(node1_position, node2_position);
        return Some((delta, new_solution));
    } else {
        return None;
    }
}

fn case_intra_edges(
    current_solution: &Vec<usize>,
    mv: MoveType,
    delta: f64,
    edge1: [usize; 2],
    edge2: [usize; 2],
    stored_moves: &mut Vec<MoveType>,
) -> Option<(f64, Vec<usize>)> {
    let [prev_node1, node1] = edge1;
    let [node2, node2_next] = edge2;
    let node1_position: usize;
    let prev_node1_position: usize;
    let node2_position: usize;
    let node2_next_position: usize;
    match current_solution.iter().position(|x| *x == node1) {
        None => return None,
        Some(position) => {
            node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == prev_node1) {
        None => return None,
        Some(position) => {
            prev_node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == node2) {
        None => return None,
        Some(position) => {
            node2_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == node2_next) {
        None => return None,
        Some(position) => {
            node2_next_position = position;
        }
    }
    if (prev_node1_position + 1 == node1_position && node2_position + 1 == node2_next_position)
        || (prev_node1_position - 1 == node1_position && node2_position - 1 == node2_next_position)
    {
        let mut new_solution = current_solution.clone();
        let sub_slice = &mut new_solution[node1_position..=node2_position];
        sub_slice.reverse();
        return Some((delta, new_solution));
    } else if (prev_node1_position - 1 == node1_position
        && node2_position + 1 == node2_next_position)
        || (prev_node1_position + 1 == node1_position && node2_position - 1 == node2_next_position)
    {
        stored_moves.push(mv);
        return None;
    } else {
        return None;
    }
}

fn add_all_moves(
    data: &Vec<DataPoint>,
    distance_matrix: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    current_solution: &Vec<usize>,
    change_edges: bool,
) {
    let moves = generate_neighborhood(current_solution, data);
    for (move_type, i, j) in moves {
        if move_type == 0 {
            if change_edges {
                add_intra_edge_move(distance_matrix, lm, current_solution, i, j);
            } else {
                add_intra_node_move(distance_matrix, lm, current_solution, i, j);
            }
        } else if move_type == 1 {
            add_inter_node_move(distance_matrix, lm, current_solution, i, j, data);
        }
    }
}

fn add_intra_edge_move(
    distance_matrix: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    current_solution: &Vec<usize>,
    i: usize,
    j: usize,
) {
    let n = current_solution.len();
    let delta = intra_edges(current_solution, i, j, distance_matrix);
    if delta > 0. {
        return;
    }
    let node1 = current_solution[i];
    let prev_node1 = current_solution[(i - 1 + n) % n];
    let node2 = current_solution[j];
    let node2_next = current_solution[(j + 1) % n];
    let intra_edge_move = IntraEdgeMove {
        delta,
        edge1: [prev_node1, node1],
        edge2: [node2, node2_next],
    };
    lm.push(Reverse(MoveType::IntraEdge(intra_edge_move)));
}
fn add_intra_node_move(
    distance_matrix: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    current_solution: &Vec<usize>,
    i: usize,
    j: usize,
) {
    let n = current_solution.len();
    let delta = intra(current_solution, i, j, distance_matrix);
    if delta > 0. {
        return;
    }
    let node1 = current_solution[i];
    let prev_node1 = current_solution[(i - 1 + n) % n];
    let node1_next = current_solution[(i + 1) % n];
    let node2 = current_solution[j];
    let prev_node2 = current_solution[(j - 1 + n) % n];
    let node2_next = current_solution[(j + 1) % n];
    let intra_node_move = IntraNodeMove {
        delta,
        node1_w_neighbors: [prev_node1, node1, node1_next],
        node2_w_neighbors: [prev_node2, node2, node2_next],
    };
    lm.push(Reverse(MoveType::IntraNode(intra_node_move)));
}
fn add_inter_node_move(
    distance_matrix: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    current_solution: &Vec<usize>,
    i: usize,
    j: usize,
    data: &Vec<DataPoint>,
) {
    let n = current_solution.len();
    let delta = inter(current_solution, i, j, distance_matrix, data);
    if delta > 0. {
        return;
    }
    let node1 = current_solution[i];
    let prev_node1 = current_solution[(i - 1 + n) % n];
    let node1_next = current_solution[(i + 1) % n];
    let inter_node_move = InterNodeMove {
        delta,
        node1_w_neighbors: [prev_node1, node1, node1_next],
        outside_node: j,
    };
    lm.push(Reverse(MoveType::InterNode(inter_node_move)));
}

fn recover_stored_moves(lm: &mut BinaryHeap<Reverse<MoveType>>, stored_moves: &mut Vec<MoveType>) {
    for mv in &mut *stored_moves {
        lm.push(Reverse(mv.clone()));
    }
    stored_moves.clear();
}

pub fn local_search_w_cached_deltas(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    change_edges: bool,
) -> Vec<usize> {
    let mut lm: BinaryHeap<Reverse<MoveType>> = BinaryHeap::new();
    let mut current_solution =
        generate_random_solution(data, starting_point_index, distance_matrix);
    add_all_moves(
        data,
        distance_matrix,
        &mut lm,
        &current_solution,
        change_edges,
    );
    loop {
        let mut best_solution: Vec<usize> = current_solution.clone();
        let mut best_delta: f64 = 0.0;
        let mut stored_moves: Vec<MoveType> = vec![];

        while !lm.is_empty() {
            let Reverse(mv) = lm.pop().unwrap();
            match apply_if_valid(&current_solution, &mut stored_moves, mv) {
                None => {}
                Some((delta, new_solution)) => {
                    best_delta = delta;
                    best_solution = new_solution;
                    break;
                }
            }
        }
        recover_stored_moves(&mut lm, &mut stored_moves);
        if best_delta < 0. {
            current_solution = best_solution;
        } else {
            break;
        }
    }
    current_solution
}

pub fn local_search_w_cached_deltas_full(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    change_edges: bool,
) -> Vec<Vec<usize>> {
    let mut lm: BinaryHeap<Reverse<MoveType>> = BinaryHeap::new();
    let mut full_solution: Vec<Vec<usize>> = Vec::new();
    let mut current_solution =
        generate_random_solution(data, starting_point_index, distance_matrix);
    full_solution.push(current_solution.clone());
    add_all_moves(
        data,
        distance_matrix,
        &mut lm,
        &current_solution,
        change_edges,
    );
    loop {
        let mut best_solution: Vec<usize> = current_solution.clone();
        let mut best_delta: f64 = 0.0;
        let mut stored_moves: Vec<MoveType> = vec![];

        while !lm.is_empty() {
            let Reverse(mv) = lm.pop().unwrap();
            match apply_if_valid(&current_solution, &mut stored_moves, mv) {
                None => {}
                Some((delta, new_solution)) => {
                    best_delta = delta;
                    best_solution = new_solution;
                    break;
                }
            }
        }
        if best_delta < 0. {
            full_solution.push(best_solution.clone());
            current_solution = best_solution;
        } else {
            break;
        }
        for mv in &stored_moves {
            lm.push(Reverse(mv.clone()));
        }
        stored_moves.clear();
    }
    full_solution
}
